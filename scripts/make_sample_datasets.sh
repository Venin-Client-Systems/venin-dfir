#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
sample_dir="$root/datasets/samples/chromium"
db_path="$sample_dir/History"
sql_path="$sample_dir/History.sql"

rm -f "$db_path" "$db_path-wal" "$db_path-shm"
sqlite3 "$db_path" < "$sql_path"

printf 'Created synthetic Chromium History database: %s\n' "$db_path"
shasum -a 256 "$db_path"
