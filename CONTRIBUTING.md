# Contributing

## Development Setup

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

## Parser Contributions

Parser PRs must include:

- registry metadata
- typed output records
- raw source values where interpretation is applied
- UTC timestamp normalization when timestamps are present
- JSON and CSV export coverage where relevant
- timeline events for temporal artefacts
- deterministic tests
- fixture documentation
- assumptions and limitations

## Evidence Safety

Do not add tests or fixtures containing real personal data, customer data, or uncontrolled forensic evidence. Use synthetic fixtures or publicly redistributable datasets with clear provenance.

## Commit Style

Use:

```text
<scope>: <intent>
```

Examples:

```text
parsers: add chromium history visit timeline events
timeline: add csv exporter
docs: document registry metadata contract
```

## Review Focus

Reviews should prioritize:

- incorrect parsing
- timestamp mistakes
- silent data loss
- schema instability
- non-determinism
- missing validation
- evidence-modifying behavior
