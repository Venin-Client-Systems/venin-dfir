# Parser Validation Strategy

## Goals

- Confirm parser correctness against deterministic fixtures.
- Preserve raw source values for independent analyst review.
- Detect schema drift in supported applications.
- Prevent timestamp conversion regressions.

## Fixture Types

- Synthetic minimal fixtures for unit tests.
- Versioned application fixtures for schema compatibility.
- Edge-case fixtures for nulls, invalid timestamps, deleted records, and unusual encodings.
- Public forensic challenge datasets only when redistribution is permitted.

## Validation Methods

- Unit tests for timestamp and field extraction.
- Golden JSON/CSV output comparisons for stable parsers.
- Differential checks against trusted tools where licensing allows.
- Corpus runs in CI for fixtures that can be redistributed.

TODO:
- Add golden output harness.
- Add parser benchmark suite.
- Add schema drift detection for Chromium History versions.
