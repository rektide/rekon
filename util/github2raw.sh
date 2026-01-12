#!/usr/bin/env bash

# Convert GitHub URLs to raw URLs
# Example:
#   Input:  https://github.com/steveyegge/beads/blob/main/README.md
#   Output: https://raw.githubusercontent.com/steveyegge/beads/main/README.md

if [ $# -eq 0 ]; then
  echo "Usage: $0 <github-url>"
  echo "Converts a GitHub URL to a raw URL"
  echo ""
  echo "Example:"
  echo "  $0 https://github.com/steveyegge/beads/blob/main/README.md"
  exit 1
fi

url="$1"

# Convert github.com to raw.githubusercontent.com
raw_url=$(echo "$url" | sed 's|github\.com|raw.githubusercontent.com|' | sed 's|/blob/|/|')

echo "$raw_url"
