//! Small read-only textures placed into shared 64 KiB default heaps.
//!
//! This is a separate allocator from the large streamed-texture pool. It only
//! admits native D3D12 descriptions that the device confirms can use the
//! 4 KiB small-resource alignment, and it never reuses a reserved range.
//!
//! Design inspiration: the small-resource pooling work in DexNom/VIctory
//! (GPL-3.0, reviewed at 8114ff779b5e1c7112b87476753dc8d0992ed97c).
//! This implementation is adapted to CGBJ's residency and host-resource rules.

use std::collections::HashMap;
use std::sync::Mutex;

use windows::Win32::Graphics::Direct3D12::*;
use windows::core::GUID;

const HEAP_BYTES: u64 = D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT as u64;
const SMALL_ALIGNMENT: u64 = D3D12_SMALL_RESOURCE_PLACEMENT_ALIGNMENT as u64;
const BACKING_HEAP_KEY: GUID = GUID::from_u128(0xc0b7_8fc0_49d2_4d37_99e7_6fbb_7a5f_01c1);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Shape {
    format: i32,
    width: u64,
    height: u32,
    mips: u16,
}

#[derive(Default)]
pub(crate) struct SmallTextures {
    allocator: Mutex<Allocator>,
}

#[derive(Default)]
struct Allocator {
    active: Option<(ID3D12Heap, u64)>,
    allocation_sizes: HashMap<Shape, Option<u64>>,
}

/// Only ordinary, single-plane color/BC texture formats are candidates.
/// The device's allocation query is the final authority on 4 KiB support.
fn supported_format(format: i32) -> bool {
    matches!(
        format,
        1..=4
            | 9..=18
            | 23..=32
            | 34..=38
            | 41..=43
            | 48..=65
            | 67
            | 70..=84
            | 87..=99
    )
}

fn candidate(desc: &D3D12_RESOURCE_DESC) -> bool {
    desc.Dimension == D3D12_RESOURCE_DIMENSION_TEXTURE2D
        && desc.Flags == D3D12_RESOURCE_FLAG_NONE
        && desc.Layout == D3D12_TEXTURE_LAYOUT_UNKNOWN
        && desc.DepthOrArraySize == 1
        && desc.SampleDesc.Count == 1
        && desc.SampleDesc.Quality == 0
        && desc.Alignment == 0
        && desc.Width > 0
        && desc.Height > 0
        && supported_format(desc.Format.0)
}

fn small_allocation(info: D3D12_RESOURCE_ALLOCATION_INFO) -> Option<u64> {
    (info.Alignment == SMALL_ALIGNMENT
        && info.SizeInBytes >= SMALL_ALIGNMENT
        && info.SizeInBytes <= HEAP_BYTES
        && info.SizeInBytes.is_multiple_of(SMALL_ALIGNMENT))
        .then_some(info.SizeInBytes)
}

impl SmallTextures {
    pub(super) fn place(
        &self,
        device: &ID3D12Device,
        desc: &D3D12_RESOURCE_DESC,
        state: D3D12_RESOURCE_STATES,
    ) -> Option<ID3D12Resource> {
        if !candidate(desc) {
            return None;
        }

        let small_desc = D3D12_RESOURCE_DESC {
            Alignment: SMALL_ALIGNMENT,
            ..*desc
        };
        let shape = Shape {
            format: small_desc.Format.0,
            width: small_desc.Width,
            height: small_desc.Height,
            mips: small_desc.MipLevels,
        };

        let mut allocator = self.allocator.lock().unwrap_or_else(|e| e.into_inner());
        let size = match allocator.allocation_sizes.get(&shape) {
            Some(size) => *size,
            None => {
                // SAFETY: the device is queried with a complete native texture description.
                let size = small_allocation(unsafe {
                    device.GetResourceAllocationInfo(0, &[small_desc])
                });
                if allocator.allocation_sizes.len() < 256 {
                    allocator.allocation_sizes.insert(shape, size);
                }
                size
            }
        }?;

        if allocator
            .active
            .as_ref()
            .is_none_or(|(_, used)| used.checked_add(size).is_none_or(|end| end > HEAP_BYTES))
        {
            let heap_desc = D3D12_HEAP_DESC {
                SizeInBytes: HEAP_BYTES,
                Alignment: HEAP_BYTES,
                Properties: D3D12_HEAP_PROPERTIES {
                    Type: D3D12_HEAP_TYPE_DEFAULT,
                    ..Default::default()
                },
                Flags: D3D12_HEAP_FLAGS(
                    D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES.0
                        | crate::host::not_resident().0,
                ),
            };
            let mut created = None;
            // SAFETY: a 64 KiB-aligned default heap restricted to non-RT/DS textures.
            unsafe { device.CreateHeap(&heap_desc, &mut created).ok()? };
            let heap = created?;

            // CGBJ creates heaps as non-resident when configured to do so.
            // Wait for both residency and card mapping before publishing this heap to other threads.
            crate::host::resident(device, &heap)
                .and_then(|()| crate::host::mapped(device))
                .ok()?;

            allocator.active = Some((heap, 0));
        }

        let (heap, offset, full) = {
            let (heap, used) = allocator.active.as_mut()?;
            let offset = *used;
            let end = offset.checked_add(size)?;
            *used = end;
            (heap.clone(), offset, end == HEAP_BYTES)
        };
        if full {
            allocator.active = None;
        }
        drop(allocator);

        let mut texture = None;
        // SAFETY: the allocator reserved an aligned, non-overlapping range inside this live heap.
        unsafe {
            device
                .CreatePlacedResource(&heap, offset, &small_desc, state, None, &mut texture)
                .ok()?;
        }
        let texture = texture?;

        // Keep the backing heap alive as long as any descriptor or command list retains the resource.
        unsafe { texture.SetPrivateDataInterface(&BACKING_HEAP_KEY, &heap).ok()? };
        Some(texture)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows::Win32::Graphics::Dxgi::Common::*;

    fn texture_desc(width: u64, height: u32, format: DXGI_FORMAT) -> D3D12_RESOURCE_DESC {
        D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            Width: width,
            Height: height,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: format,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            ..Default::default()
        }
    }

    #[test]
    fn candidates_are_limited_to_small_read_only_single_slice_textures() {
        let valid = texture_desc(128, 128, DXGI_FORMAT_BC7_UNORM);
        assert!(candidate(&valid));

        let invalid = [
            D3D12_RESOURCE_DESC { Flags: D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET, ..valid },
            D3D12_RESOURCE_DESC { Flags: D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS, ..valid },
            D3D12_RESOURCE_DESC { DepthOrArraySize: 2, ..valid },
            D3D12_RESOURCE_DESC {
                SampleDesc: DXGI_SAMPLE_DESC { Count: 4, Quality: 0 },
                ..valid
            },
            D3D12_RESOURCE_DESC { Width: 0, ..valid },
            D3D12_RESOURCE_DESC { Format: DXGI_FORMAT_NV12, ..valid },
        ];
        assert!(invalid.iter().all(|desc| !candidate(desc)));
    }

    #[test]
    fn allocation_query_must_confirm_small_alignment_and_fit_one_heap() {
        assert_eq!(
            small_allocation(D3D12_RESOURCE_ALLOCATION_INFO {
                SizeInBytes: SMALL_ALIGNMENT,
                Alignment: SMALL_ALIGNMENT,
            }),
            Some(SMALL_ALIGNMENT)
        );
        assert_eq!(
            small_allocation(D3D12_RESOURCE_ALLOCATION_INFO {
                SizeInBytes: HEAP_BYTES + SMALL_ALIGNMENT,
                Alignment: SMALL_ALIGNMENT,
            }),
            None
        );
        assert_eq!(
            small_allocation(D3D12_RESOURCE_ALLOCATION_INFO {
                SizeInBytes: SMALL_ALIGNMENT,
                Alignment: HEAP_BYTES,
            }),
            None
        );
    }
}
