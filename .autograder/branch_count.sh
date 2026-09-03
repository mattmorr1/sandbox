#!/bin/bash

# Git Branch Analysis Script
# Usage: ./analyze_branches.sh [n_branches]

show_usage() {
    echo "Usage: $0 [n_branches]"
    echo ""
    echo "Checks how many unique branches have commits and prints results."
    echo "Exits 0 if at least n_branches exist, otherwise exits 1."
    echo ""
    echo "Example:"
    echo "  $0 5   # Require at least 5 branches with commits"
}

# Get all unique branch names from commit history that had at least one commit
get_unique_branches() {
    git log --all --pretty=format:"%D" \
      | grep -o '[^,)]*' \
      | sed 's/^ *//;s/ *$//' \
      | grep -v '^$' \
      | sed 's/HEAD -> //' \
      | sed 's/origin\///' \
      | grep -v '^HEAD$' \
      | grep -v 'feedback' \
      | sort -u
}

# Parse argument
if [ $# -ne 1 ]; then
    show_usage
    exit 1
fi

N_BRANCHES=$1
if ! [[ $N_BRANCHES =~ ^[0-9]+$ ]]; then
    echo "Error: Argument must be a number"
    show_usage
    exit 1
fi

# Count branches
UNIQUE_BRANCHES=$(get_unique_branches)
BRANCH_COUNT=$(echo "$UNIQUE_BRANCHES" | wc -l)

# Always print status
echo "🔎 Found $BRANCH_COUNT branches with commits"
echo "$UNIQUE_BRANCHES"

# Decide success/failure
if [ $BRANCH_COUNT -ge $N_BRANCHES ]; then
    echo "✅ Success: At least $N_BRANCHES branches with commits"
    exit 0
else
    echo "❌ Failure: Only $BRANCH_COUNT branches found, need at least $N_BRANCHES"
    exit 1
fi
