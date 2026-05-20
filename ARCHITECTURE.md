# Architecture

## Decision

VENIN DFIR uses a Rust + Python architecture.

Rust is the canonical implementation layer for:

- CLI commands
- forensic parsers
- binary parsing
- SQLite parsing
- timestamp normalization
- hashing and file identification
- output serialization
- timeline reconstruction
- registry metadata

Python is a secondary analyst layer for:

- repeatable post-processing over VENIN JSON/CSV
- notebooks
- validation scripts
- case-specific automation
- exploratory parser research before promotion into Rust

The first implemented parser, `browser_history.chromium`, is Rust-native.

## Why Rust + Python

### Rust

Rust is the best default for VENIN DFIR's long-term core because forensic tooling regularly needs safe handling of untrusted files, binary formats, large datasets, and strict reproducibility.

Strengths:

- Memory safety without garbage collection.
- High performance for large artefact collections.
- Good cross-platform support for Windows, macOS, and Linux.
- Strong CLI ergonomics through `clap`.
- Reliable SQLite access through `rusqlite`.
- Precise data modeling with enums and typed structs.
- Strong serialization support through `serde`.
- Good packaging story through single binaries.
- Contributor-friendly compiler feedback.

Tradeoffs:

- Parser authors need more type discipline than Python.
- Some DFIR ecosystems publish reference research in Python first.
- Native build toolchains need care on macOS and Windows.

### Python

Python remains valuable for analyst workflows. It is excellent for quick triage scripts, notebooks, ad hoc reporting, and validating parser assumptions against research datasets.

In VENIN DFIR, Python is deliberately not the canonical parser runtime. That avoids dependency sprawl, interpreter packaging issues, and inconsistent runtime behavior across case machines.

### Go

Go is excellent for operational tooling, easy static binaries, and straightforward concurrency. It was considered seriously.

Reasons Go was not selected as the core:

- Rust provides stronger memory-safety guarantees for hostile binary parsing.
- Rust's type system better represents artefact schemas and parser invariants.
- Rust avoids garbage collector pauses for high-volume parsing.
- Rust has strong binary parsing ecosystem patterns.

Go may still be appropriate for future remote acquisition helpers or agents if operational simplicity becomes more important than parser expressiveness.

### Pure Python

Pure Python was rejected for the canonical toolkit. Python is excellent for research and analysis, but less ideal for high-volume forensic parsing, reproducible packaging, and memory-safe handling of arbitrary untrusted artefacts.

## Monorepo Layout

```text
cmd/venin-cli              CLI entry point
core/venin-core            registry and tool metadata
shared/venin-shared        timestamps, hashing, file identification
parsers/venin-parsers      parser implementations and scaffolds
timeline/venin-timeline    normalized timeline model and exporters
acquisition/               acquisition planning and future collection modules
windows/                   Windows workflow support
ios/                       iOS backup workflow support
android/                   Android workflow support
mobile/                    cross-mobile workflow support
artifacts/                 output schemas and artefact model docs
datasets/                  synthetic sample datasets
testdata/                  deterministic parser fixtures
cases/                     case workspace templates
reports/                   report templates
tools/python               analyst-side Python helpers
```

## Parser Contract

Every parser should:

- open source artefacts read-only where technically possible
- avoid modifying input evidence
- preserve raw fields that affect interpretation
- normalize timestamps to UTC
- emit JSON and CSV
- emit timeline events for temporal findings
- document assumptions and limitations
- expose registry metadata
- include deterministic tests

## Tool Registry

`venin-core` owns `ToolRegistry` and `ToolMetadata`.

The registry records:

- stable tool ID
- display name
- semantic version
- parser or workflow kind
- maturity status
- supported artefacts
- expected input types
- output formats
- output schemas
- evidence handling notes

`venin-parsers::register_builtin_tools` registers built-in modules. Future plugin expansion should use the same metadata shape, either through dynamic manifest loading or compiled feature crates.

## CLI Design

Primary command groups:

```text
venin registry list
venin parse chrome-history
venin timeline build
venin acquire memory
venin mobile ios-backup
```

The CLI should remain stable and scriptable. Output should be written to case/output directories, not hidden application state.

## Timeline Model

Timeline events are normalized to UTC and include:

- timestamp
- source tool
- artefact type
- event type
- description
- source path
- evidence ID
- structured fields

The event model intentionally keeps parser-specific fields in a JSON map so the common timeline schema can evolve without flattening every artefact-specific property into the core type.

## Logging

Logging uses `tracing`.

Principles:

- log parser lifecycle and counts
- avoid dumping evidence content to logs
- support human-readable and JSON logs
- keep logs deterministic enough for case notes

## Packaging

Initial packaging target is a Rust binary:

```bash
cargo build --release -p venin-cli
```

Future packaging:

- GitHub release binaries for macOS, Linux, and Windows.
- Homebrew tap.
- Signed release checksums.
- Optional Python package for `tools/python`.

## Future Architecture Evolution

Planned evolution:

- Plugin manifest loader for third-party parser metadata.
- Schema versioning for parser outputs.
- Corpus-based parser validation.
- Benchmarks for large artefact sets.
- Optional Rust dynamic plugin ABI only if static plugin crates become limiting.
- Python notebooks that consume only exported datasets, not live evidence.
