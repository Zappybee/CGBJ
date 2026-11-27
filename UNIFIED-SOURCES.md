# Unified source collection

This repository consolidates the CGBJ runtime and the requested Xbox Series compatibility projects into one Git repository.

## Main project

- **CGBJ**: the active root workspace. It is the fork of `DoubleGhosty/cool_fox` and remains the default build target.

## Independently buildable engines and variants

| Directory | Source | License | Build boundary |
|---|---|---|---|
| `engines/kelvaroq/` | `stellarr-studio/kelvaroq` | GPL-3.0 | Independent Kelvaroq workspace |
| `engines/xcl/` | `xcl-dev/xcl` | GPL-2.0-only | Independent XCL workspace; deliberately isolated from GPL-3.0 components |
| `engines/victory-7900xtx/` | `avatarmanz/VIctory-7900XTX` | GPL-3.0 | Independent VIctory variant |
| `engines/dexnom-victory/` | `DexNom/VIctory` | GPL-3.0 | Independent VIctory variant |
| `engines/belobug-victory-amd/` | `Belobug/VIctory-Belobug-AMD` | GPL-3.0 | Independent VIctory variant |
| `engines/fxceless-victory-fork/` | `fxceless/victory-fork` | GPL-3.0 | Independent VIctory variant |
| `engines/vitoriabr/` | `uiliannn/VItoriaBr` | GPL-3.0 | Independent VIctory variant |
| `engines/marvisak-victory/` | `Marvisak/VIctory` | GPL-3.0 | Upstream VIctory baseline |

`DoubleGhosty/cool_fox` is already the root CGBJ source tree, not an additional duplicate directory.

## Why the engines remain separate

The VIctory-family projects share substantial code but have diverged in their GPU memory, residency, descriptor, shader, and ray-tracing implementations. Their complete source snapshots are kept under separate directories so that one implementation does not overwrite another before its changes can be reconciled and tested. Each snapshot retains its own `Cargo.toml`, lockfile, README, and licence.

XCL is a separate GPL-2.0-only project. It is included as a standalone workspace and is not linked into, copied into, or treated as part of the GPL-3.0 CGBJ runtime.

## Data exclusions

Binary game assets, executables, shader binaries, `.bin` format tables, `.dxil` files, and captured runtime diagnostics are not included in these snapshots. Each engine's source and build instructions are preserved; use only game files you lawfully supply locally and keep them outside this repository.

## Provenance

These are source snapshots of the default branches at the exact commits listed below. Their upstream histories are not rewritten into one synthetic commit graph.

| Source | Imported commit |
|---|---|
| `stellarr-studio/kelvaroq` | `e062a82e04853bbcae281d166980e0a9b65c975a` |
| `xcl-dev/xcl` | `8b957bec6f82495aec940944c6bf47faa02db590` |
| `avatarmanz/VIctory-7900XTX` | `014d52dedf837d1ac660a0bd256c4704be9eeb40` |
| `DexNom/VIctory` | `8114ff779b5e1c7112b87476753dc8d0992ed97c` |
| `Belobug/VIctory-Belobug-AMD` | `4eb6cfdf9e61cb112f1f811344ec0dd309eddd6c` |
| `fxceless/victory-fork` | `674290601fb9dba441e711cc951a8950ff08ddc6` |
| `uiliannn/VItoriaBr` | `1361e190b7f46df996f00ea1b540ecf66785127f` |
| `Marvisak/VIctory` | `63bcca43c6a00a758510398fa43096ce7e4f7df0` |

## Build

Build CGBJ from the repository root using its existing instructions. Build a standalone engine from its own directory, following that engine's README and pinned toolchain. The source import itself does not claim that all eight independent workspaces have passed a combined build or runtime test.
