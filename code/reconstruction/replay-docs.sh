#!/usr/bin/env bash
# replay-docs.sh — rebuild a docs-only line on top of an upstream tip.
#
# For every commit on <LINE> not contained in <BASE>:
#   - code-only commits are skipped (they stay on the original line);
#   - docs-only commits are duplicated and rebased onto the growing tip;
#   - mixed commits are duplicated, their docs paths split out directly at
#     the tip (jj split -A), and the code remainder abandoned.
#
# Originals are never rewritten — every operation targets a duplicate, so the
# source line and any bookmarks on it (e.g. a dated snapshot) are immutable.
# Author timestamps are preserved natively by jj rewrite operations; that is
# verified per commit as the script runs.
#
# Usage: replay-docs.sh <repo-dir> <base-revset> <line-revset>
set -euo pipefail

REPO=${1:?repo dir}
BASE=${2:?base revset, e.g. main@origin}
LINE=${3:?line revset, e.g. systemd-20260903}

cd "$REPO"

# Docs predicate: .design/, docs/, doc/, design/ subtrees plus any *.md/*.mdx.
is_docs_path() {
  case "$1" in
    .design/*|docs/*|doc/*|design/*|*.md|*.mdx) return 0 ;;
    *) return 1 ;;
  esac
}

docs_paths_of() { # rev -> stdout: docs paths touched by rev
  jj diff -r "$1" --summary | while read -r _st path; do
    if is_docs_path "$path"; then printf '%s\n' "$path"; fi
  done
}

has_code_paths() { # rev -> rc 0 if rev touches non-docs paths
  local p
  while read -r p; do
    is_docs_path "$p" || return 0
  done < <(jj diff -r "$1" --summary | awk '{print $2}')
  return 1
}

docs_message() { # rev short -> description for the docs half of a mixed commit
  local rev=$1 short=$2 full first rest
  full=$(jj log -r "$rev" --no-graph -T 'description')
  first=$(head -1 <<<"$full")
  rest=$(tail -n +2 <<<"$full")
  printf 'docs: %s (docs portion of %s)\n\n%s\n(docs portion of mixed commit %s — the code half stays on the source line.)\n' \
    "$first" "$short" "${rest:-}" "$short"
}

TIP=$(jj log -r "$BASE" --no-graph -T 'commit_id ++ "\n"')
echo "base tip: $TIP ($(jj log -r "$BASE" --no-graph -T 'description.first_line()'))"
echo

n=0 kept=0
for REV in $(jj log -r "$BASE..$LINE" --no-graph -T 'commit_id ++ "\n"' | tac); do
  n=$((n + 1))
  mapfile -t DOCS_PATHS < <(docs_paths_of "$REV")
  ALL_PATHS=$(jj diff -r "$REV" --summary | awk '{print $2}' | grep -c . || true)
  SHORT=${REV:0:12}
  SUBJECT=$(jj log -r "$REV" --no-graph -T 'description.first_line()')
  ORIG_DATE=$(jj log -r "$REV" --no-graph -T 'author.timestamp().format("%Y-%m-%d %H:%M")')

  if [ "${#DOCS_PATHS[@]}" -eq 0 ] && [ "${ALL_PATHS:-0}" -ne 0 ]; then
    echo "[$n] $REV $ORIG_DATE SKIP (code-only): $SUBJECT"
    continue
  fi

  # jj duplicate prints: "Duplicated <old> as <change-id> <commit-id> <desc>" (on stderr)
  DUPLINE=$(jj duplicate "$REV" 2>&1 | awk '/^Duplicated/')
  DUP=$(awk '{print $5}' <<<"$DUPLINE")
  DUP_CHANGE=$(awk '{print $4}' <<<"$DUPLINE")
  if has_code_paths "$REV"; then
    # Plain split (NO -A/-B/-o!): selected docs changes stay in the duplicate,
    # the code remainder becomes its child. `jj split -A <dest>` would REBASE
    # dest's descendants (i.e. the entire source line) on top of the extracted
    # part — dragging bookmarks along. Duplicate-then-split keeps everything
    # else in place; the remainder is abandoned before the rebase so the
    # selected part is a leaf and rebase touches nothing else.
    OUT=$(jj split -r "$DUP" -m "$(docs_message "$REV" "$SHORT")" "${DOCS_PATHS[@]}" 2>&1) || { echo "split failed for $REV:"; echo "$OUT"; exit 1; }
    if grep -qi conflict <<<"$OUT"; then echo "CONFLICT replaying $REV — aborting"; exit 1; fi
    # In a plain split the selected part KEEPS the duplicate's change-id.
    SEL=$(jj log -r "$DUP_CHANGE" --no-graph -T 'commit_id ++ "\n"')
    # defensive: the docs part must touch only docs paths
    while read -r p; do
      is_docs_path "$p" || { echo "selected part of $REV touches non-docs path $p — aborting"; exit 1; }
    done < <(jj diff -r "$SEL" --summary | awk '{print $2}')
    REM=$(jj log -r "children($SEL)" --no-graph -T 'commit_id ++ "\n"')
    if [ -n "$REM" ]; then
      jj abandon "$REM" >/dev/null
    fi
    OUT=$(jj rebase -r "$SEL" -d "$TIP" 2>&1)
    if grep -qi conflict <<<"$OUT"; then echo "CONFLICT replaying $REV — aborting"; exit 1; fi
    TIP=$(jj log -r "$DUP_CHANGE" --no-graph -T 'commit_id ++ "\n"')
    echo "[$n] $REV $ORIG_DATE MIXED -> docs part $TIP (${#DOCS_PATHS[@]} doc paths): $SUBJECT"
  else
    OUT=$(jj rebase -r "$DUP" -d "$TIP" 2>&1)
    if grep -qi conflict <<<"$OUT"; then echo "CONFLICT replaying $REV — aborting"; exit 1; fi
    # rebase rewrites the commit id; the change-id is stable — re-resolve.
    TIP=$(jj log -r "$DUP_CHANGE" --no-graph -T 'commit_id ++ "\n"')
    echo "[$n] $REV $ORIG_DATE DOCS   -> $TIP: $SUBJECT"
  fi
  kept=$((kept + 1))

  NEW_DATE=$(jj log -r "$TIP" --no-graph -T 'author.timestamp().format("%Y-%m-%d %H:%M")')
  if [ "$NEW_DATE" != "$ORIG_DATE" ]; then
    echo "  !! author date changed: $ORIG_DATE -> $NEW_DATE"
    exit 1
  fi
done

echo
echo "replayed $kept of $n commits onto $BASE"
echo "FINAL TIP: $TIP"
