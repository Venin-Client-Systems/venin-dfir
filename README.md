# VENIN DFIR

VENIN DFIR is a CLI-first digital forensics toolkit for evidence acquisition support, artefact parsing, timeline reconstruction, and repeatable investigative automation.

The project is intentionally not GUI-first. It follows a small-tool philosophy: each parser should do one artefact family well, emit deterministic structured output, and integrate with a shared timeline model.

## Architecture

The chosen stack is Rust + Python:

- Rust owns canonical parsers, the CLI, registry, timestamp normalization, hashing, exports, and timeline generation.
- Python is reserved for analyst-side workflow helpers, notebooks, validation, and repeatable post-processing over VENIN JSON/CSV output.

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full Rust vs Go vs Python decision.

## Current Working Parser

`browser_history.chromium` is the first implemented parser.

It parses Chromium-family `History` SQLite databases and extracts:

- URL
- title
- visit count
- typed count
- hidden flag
- raw Chromium timestamp
- normalized UTC timestamp
- visit-level timeline events when the `visits` table is present

Supported browser families include Chrome, Chromium, Edge, and Brave history databases that use the Chromium schema.

## Build

```bash
cd /Users/georgeatkinson/Developer/digital-forensics/venin-dfir
cargo build --workspace
```

On macOS, Rust builds that link native dependencies require the Xcode command line tools license to be accepted.

## Run Tests

```bash
cargo test --workspace
```

## CLI Examples

List registered tools:

```bash
cargo venin -- registry list
```

Parse a Chromium History database:

```bash
cargo venin -- parse chrome-history \
  --input datasets/samples/chromium/History \
  --evidence-id CASE001-EV001 \
  --format both \
  --output-dir output
```

Build a unified timeline from normalized timeline JSON:

```bash
cargo venin -- timeline build \
  --input output/timelines/chromium_history.timeline.json \
  --format both \
  --output-dir output
```

## Repository Map

- `cmd/venin-cli` - CLI entry point.
- `core/venin-core` - tool registry and shared tool metadata.
- `parsers/venin-parsers` - forensic parsers and parser scaffolds.
- `timeline/venin-timeline` - normalized event model and JSON/CSV exports.
- `shared/venin-shared` - timestamp, hashing, and file identification utilities.
- `acquisition/` - acquisition planning scaffolding.
- `windows/`, `ios/`, `android/`, `mobile/` - platform workflow crates and future expansion.
- `artifacts/` - output schema definitions.
- `cases/` - forensic case workspace template.
- `datasets/` and `testdata/` - synthetic fixtures and parser validation inputs.
- `reports/` - markdown report templates.
- `tools/python` - analyst-side Python helpers for VENIN exports.

## Case Workspaces

Create a case directory with:

```bash
scripts/new_case.sh CASE002
```

Case layout:

```text
cases/CASE002/
  evidence/
  acquisitions/
  parsed/
  timelines/
  reports/
  notes/
```

## Add A Parser

1. Add a parser module under `parsers/venin-parsers/src`.
2. Define typed output records with `serde`.
3. Preserve raw artefact values beside normalized values.
4. Emit `TimelineEvent` values for temporal records.
5. Add metadata to `register_builtin_tools`.
6. Add deterministic fixtures and unit tests.
7. Document the parser in `TOOL_REGISTRY.md`.

## License

Recommended license is dual `MIT OR Apache-2.0`, matching common Rust open-source practice and making contribution terms clear for both individuals and organizations.
