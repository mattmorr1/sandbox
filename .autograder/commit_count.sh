#!/usr/bin/env bash
# .autograder/commit_count.sh
set -euo pipefail

# Usage:
#   bash .autograder/commit_count.sh 3
#   MIN=3 bash .autograder/commit_count.sh

MIN="${1:-${MIN:-0}}"

# List of authors to exclude (bot and staff)
EXCLUDED_AUTHORS=(
  "github-classroom[bot]"
  "trgardos"
  "zgentile"
)

# Validate MIN
if ! [[ "$MIN" =~ ^[0-9]+$ ]]; then
  echo "MIN must be a non-negative integer; got: '$MIN'" >&2
  exit 2
fi

# Ensure we're in a git repo
if ! git rev-parse --git-dir >/dev/null 2>&1; then
  echo "Not a git repository (are you running inside the checkout?)" >&2
  exit 1
fi

# Warn if shallow (runner must checkout with fetch-depth: 0 for full history)
if [ -f "$(git rev-parse --git-dir)/shallow" ]; then
  echo "Warning: shallow clone detected; commit count may be incomplete." >&2
fi

# Build a grep -vE pattern to exclude authors
build_exclude_grep() {
  local pattern
  pattern=$(printf '%s\n' "${EXCLUDED_AUTHORS[@]}" | sed 's/[.[\*^$]/\\&/g' | paste -sd '|' -)
  echo "$pattern"
}

# Count commits excluding specified authors
# git log --format includes author name; grep -vE filters out excluded authors
EXCLUDE_GREP=$(build_exclude_grep)
COUNT=$(git log --all --no-merges --format="%an" 2>/dev/null \
  | grep -vcE "^(${EXCLUDE_GREP})$" \
  | tr -d ' ')

if [ "$COUNT" -ge "$MIN" ]; then
  echo "✅ Found $COUNT commits (min $MIN) — PASS"
  exit 0
else
  echo "❌ Found $COUNT commits (min $MIN) — FAIL"
  exit 1
fi
