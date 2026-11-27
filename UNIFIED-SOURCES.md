# Unified source collection

The parent repository is the CGBJ fork of `DoubleGhosty/cool_fox`. The other requested projects are pinned Git submodules under `engines/`, so one recursive clone checks out all projects while preserving each upstream's own commit history.

## Main project

- **CGBJ**: root workspace at `Zappybee/CGBJ`, forked from `DoubleGhosty/cool_fox`. Root commit baseline: `69718c00fb59ada793a9eb57ebdfb7b80b9c676a`.

## Pinned engine submodules

| Directory | Upstream repository | License | Pinned commit |
|---|---|---|---|
| `engines/kelvaroq/` | `stellarr-studio/kelvaroq` | GPL-3.0 | `e062a82e04853bbcae281d166980e0a9b65c975a` |
| `engines/xcl/` | `xcl-dev/xcl` | GPL-2.0-only | `8b957bec6f82495aec940944c6bf47faa02db590` |
| `engines/victory-7900xtx/` | `avatarmanz/VIctory-7900XTX` | GPL-3.0 | `014d52dedf837d1ac660a0bd256c4704be9eeb40` |
| `engines/dexnom-victory/` | `DexNom/VIctory` | GPL-3.0 | `8114ff779b5e1c7112b87476753dc8d0992ed97c` |
| `engines/belobug-victory-amd/` | `Belobug/VIctory-Belobug-AMD` | GPL-3.0 | `4eb6cfdf9e61cb112f1f811344ec0dd309eddd6c` |
| `engines/fxceless-victory-fork/` | `fxceless/victory-fork` | GPL-3.0 | `674290601fb9dba441e711cc951a8950ff08ddc6` |
| `engines/vitoriabr/` | `uiliannn/VItoriaBr` | GPL-3.0 | `1361e190b7f46df996f00ea1b540ecf66785127f` |
| `engines/marvisak-victory/` | `Marvisak/VIctory` | GPL-3.0 | `63bcca43c6a00a758510398fa43096ce7e4f7df0` |

`DoubleGhosty/cool_fox` is represented by the root CGBJ fork; it is not added a second time as a duplicate submodule.

## Why they are separate submodules

The VIctory-family projects share substantial code but have diverged in GPU memory, residency, descriptor, shader, and ray-tracing implementations. Keeping each variant pinned as its own submodule prevents a blind overlay from replacing one implementation with another before changes can be reconciled and tested.

XCL is licensed GPL-2.0-only, while the CGBJ/VIctory-family code is GPL-3.0-or-later. XCL is therefore kept as a separate program and repository boundary; it is not copied into or linked with the GPL-3.0 runtime.

## Clone and build

```powershell
git clone --recurse-submodules https://github.com/Zappybee/CGBJ.git
cd CGBJ
```

For an existing clone:

```powershell
git submodule update --init --recursive
```

Build CGBJ from the repository root using its README. Build another engine from its own submodule directory using that project's README and pinned toolchain. The submodule registration is complete, but no claim is made that the eight independent engine workspaces have passed one combined build or runtime test.

## Data handling

Game assets, executable dumps, shader binaries, local format tables, and runtime diagnostic captures are not added to the parent repository. Submodules retain their own upstream asset policies. Use only game files you lawfully supply locally and keep those files outside the repository.
