# VENIN DFIR Claude Instructions

VENIN DFIR is a Rust-first digital forensics toolkit. Treat this as forensic software, not generic cybersecurity tooling.

## Architecture

- Rust owns canonical parsers, CLI behavior, registry metadata, timestamp conversion, hashing, file identification, timeline generation, and JSON/CSV exports.
- Python under `tools/python` is for analyst-side helpers over generated VENIN outputs only.
- Keep the CLI scriptable and stable.
- Do not add GUI-first workflows.

## Forensic Rules

- Never modify input evidence.
- Preserve raw artefact values beside normalized interpretations.
- Normalize timestamps to UTC.
- Avoid local timezone assumptions.
- Keep parser output deterministic for identical input.
- Do not commit real personal data, customer data, secrets, or uncontrolled forensic evidence.
- Use synthetic fixtures or clearly redistributable public datasets only.

## Rust Workflow

Run these before finishing code changes:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

For this macOS workstation, Rust builds may require the Xcode license to be accepted before linker-based checks can run.

## Parser Contributions

Every parser should include:

- registry metadata in `venin-parsers::register_builtin_tools`
- typed `serde` output records
- raw and normalized timestamp fields where applicable
- timeline events for temporal artefacts
- JSON and CSV export support where useful
- deterministic tests
- fixture provenance notes
- documented assumptions and limitations

## GitHub Action

This repository includes:

- `.github/workflows/claude.yml` for `@claude` issue and PR comment handling
- `.github/workflows/claude-review.yml` for automatic pull request review

Repository setup still requires:

- Claude GitHub App installed on the GitHub repository
- `ANTHROPIC_API_KEY` repository secret configured
- Contents, Issues, Pull requests set to read/write for the app/action
