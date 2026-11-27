# Independent research, implementation, and credits

CGBJ is developed as one unified compatibility engine. The projects listed below are research references, not source-code components to be pasted or mechanically combined.

## Integration rule

For every external project:

1. Study its behavior, architecture, design trade-offs, tests, and documented results.
2. Identify the general engineering idea that may solve a CGBJ problem.
3. Write a CGBJ-native design from first principles. Use CGBJ's own types, ownership model, synchronization, diagnostics, error handling, and naming.
4. Implement the design independently. Do not copy source files, functions, code blocks, or mechanically translate upstream code.
5. Add tests that express the required behavior, including edge cases and failure paths.
6. Benchmark and validate on actual supported hardware before claiming an improvement.
7. Credit the project and its owner for the idea or research direction that informed the work. Be explicit about what was studied and what was independently implemented.

A credit is not a substitute for a compatible license. Before importing any upstream implementation, patch, test, generated output, or substantial code-derived expression, review its license and provenance. Keep license-incompatible projects as research references unless an independently written implementation has been reviewed for legal and technical compatibility.

## Research references and acknowledgements

| Project / owner | Research area | How the work may inform CGBJ |
| --- | --- | --- |
| [Kelvaroq — stellarr-studio](https://github.com/stellarr-studio/kelvaroq) | GPU heap range management, memory residency, adapter/runtime structure | Study allocator invariants, fragmentation handling, residency observations, and test methodology. Design CGBJ-native mechanisms rather than transplanting its allocator. |
| [VIctory-7900XTX — AvatarManz](https://github.com/avatarmanz/VIctory-7900XTX) | GPU memory behavior, shader-fault handling, texture streaming and VRAM pressure | Study fault patterns, bounded work, diagnostics, and memory-pressure behavior. Re-derive policies for CGBJ's own resource and memory model. |
| [cool_fox — DoubleGhosty](https://github.com/DoubleGhosty/cool_fox) | Existing CGBJ/VIctory codebase and compatibility-layer architecture | Acknowledge the upstream foundation from which the current CGBJ repository descends. Continue evolving the runtime as a coherent project. |
| [XCL — xcl-dev](https://github.com/xcl-dev/xcl) | Console graphics API emulation, GPU and VRAM modeling | Study its independent architecture, API behavior, and compatibility decisions. It remains a separate project and license boundary. |
| [VIctory — DexNom](https://github.com/DexNom/VIctory) | Resource allocation, small texture placement, ray-tracing policy and tests | Study allocation constraints, lifetime rules, fallback behavior, and test cases. Any CGBJ implementation must be independently designed around CGBJ's residency and resource lifetime. |
| [VIctory-Belobug-AMD — Belobug](https://github.com/Belobug/VIctory-Belobug-AMD) | Descriptor tracking, write-site instrumentation, heap synchronization | Study ways to reduce descriptor scanning and measure the synchronization cost. Do not transplant title-specific patches; derive a CGBJ-native tracking design. |
| [victory-fork — fxceless](https://github.com/fxceless/victory-fork) | Incremental bindless-heap scanning, scheduling, and worker planning | Study changed-slot tracking, stale-entry detection, bounded revisits, and synchronization costs. Reconcile the ideas with CGBJ's own write-watch and BufferSweep invariants. |
| [VItoriaBr — Uilian](https://github.com/uiliannn/VItoriaBr) | Shader instruction coverage, image operations, packed arithmetic and DXIL lowering | Use as a behavioral reference for instruction cases and regression scenarios. Re-derive lowering in CGBJ and prove it with opcode-level tests. |
| [VIctory — Marvisak](https://github.com/Marvisak/VIctory) | Original VIctory architecture, runtime behavior and shader/GPU implementation | Acknowledge the upstream project lineage and use its public behavior as a comparison point during CGBJ development. |

## Implementation provenance

The initial integration work in the accompanying pull request is new CGBJ-side code and tests written for this repository. It is informed by studying upstream behavior; it is not intended to be a drop-in copy of an upstream file. The source-specific concepts and references are identified in the pull request and here.

This record is an engineering acknowledgement, not a claim that every listed idea has been integrated, independently validated, or shown to improve performance. Each integration must be tracked with its own implementation, tests, benchmark evidence, and review status.

## Quality bar

CGBJ does not aim to maximize the number of features collected. It aims to select the ideas that survive correctness testing, resource-lifetime review, hardware validation, and repeatable performance measurement. A feature that is not safe or demonstrably useful stays out of the runtime.
