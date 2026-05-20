# Forensic Principles

VENIN DFIR is built around evidentiary handling, not generic security automation.

## Integrity

- Never modify original evidence.
- Prefer parsing copied artefacts or mounted read-only evidence.
- Record source paths, hashes, parser versions, and command invocations.
- Keep raw values beside normalized interpretations.

## Reproducibility

- Parser output should be deterministic for the same input.
- Timestamp conversions must be tested against fixed values.
- Output schemas must be versioned.
- Report templates should separate facts, interpretation, and assumptions.

## Transparency

- Analysts should be able to inspect every parser assumption.
- Lossy transformations must be documented.
- Unsupported schema variants should fail clearly or emit explicit warnings.

## Timeline Discipline

- Normalize timestamps to UTC.
- Preserve original timestamp values and source fields.
- Avoid silently applying local timezone assumptions.
- Treat absent, zero, or invalid timestamps as nullable evidence facts.

## Chain Of Custody Awareness

VENIN DFIR is not a chain-of-custody system by itself, but it should support chain-of-custody practice by producing repeatable outputs and preserving references to evidence identifiers.

TODO:
- Add acquisition manifests.
- Add command invocation capture.
- Add output manifest hashing.
- Add signed report bundle support.
