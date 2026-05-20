# Tool Registry

The registry is the discoverability and integration layer for parsers and workflows.

## Current Registered Tools

| Tool ID | Status | Purpose |
| --- | --- | --- |
| `browser_history.chromium` | Experimental | Parse Chromium-family History SQLite databases. |
| `windows.usb_artifacts` | Scaffold | Correlate USB registry and setup artefacts. |
| `windows.recent_files` | Scaffold | Parse RecentDocs and Jump List-style recent file artefacts. |
| `windows.lnk` | Scaffold | Parse Windows shell link files. |
| `windows.prefetch` | Scaffold | Parse Windows Prefetch files. |
| `mobile.ios_backup` | Scaffold | Traverse iOS backup manifests and mapped files. |
| `mobile.android_sqlite` | Scaffold | Parse Android application SQLite artefacts. |

## Metadata Requirements

Every tool must define:

- `id`
- `display_name`
- `version`
- `kind`
- `status`
- `supported_artifacts`
- `input_types`
- `output_formats`
- `output_schemas`
- `evidence_notes`

## ID Naming

Use stable dotted IDs:

```text
domain.artefact
domain.artefact.variant
```

Examples:

```text
browser_history.chromium
windows.prefetch
windows.lnk
mobile.ios_backup
mobile.android_sqlite
```

## Plugin Expansion

Future plugin support should use the same registry metadata. The likely path is:

1. Static first-party crates.
2. Manifest-discovered external parser packages.
3. Optional compiled plugin crates if versioning and safety requirements are clear.

Dynamic plugins should not be introduced until parser schema compatibility, trust boundaries, and release signing are defined.
