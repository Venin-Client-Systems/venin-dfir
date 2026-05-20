#!/usr/bin/env bash
set -euo pipefail

case_id="${1:?usage: scripts/new_case.sh CASE_ID}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
case_dir="$root/cases/$case_id"

mkdir -p \
  "$case_dir/evidence" \
  "$case_dir/acquisitions" \
  "$case_dir/parsed" \
  "$case_dir/timelines" \
  "$case_dir/reports" \
  "$case_dir/notes"

if [[ ! -f "$case_dir/reports/investigation_report.md" ]]; then
  cp "$root/reports/templates/investigation_report.md" "$case_dir/reports/investigation_report.md"
fi

if [[ ! -f "$case_dir/notes/analyst_notes.md" ]]; then
  printf '# Analyst Notes\n\n' > "$case_dir/notes/analyst_notes.md"
fi

printf 'Created case workspace: %s\n' "$case_dir"
