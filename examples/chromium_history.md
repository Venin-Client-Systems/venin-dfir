# Chromium History Example

Generate the synthetic dataset:

```bash
scripts/make_sample_datasets.sh
```

Parse it:

```bash
cargo venin -- parse chrome-history \
  --input datasets/samples/chromium/History \
  --evidence-id CASE001-EV001 \
  --format both \
  --output-dir output
```

Expected outputs:

```text
output/parsed/browser_history/chromium_history.json
output/parsed/browser_history/chromium_history.csv
output/timelines/chromium_history.timeline.json
output/timelines/chromium_history.timeline.csv
```
