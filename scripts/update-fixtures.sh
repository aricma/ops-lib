#!/usr/bin/env bash
# Keep tests/fixtures/ops in sync with the latest OPS main.
#
#   update-fixtures.sh check   exit 0 if up to date, 1 if stale (CI gate)
#   update-fixtures.sh update  pull the latest fixtures + SOURCE_COMMIT
#
# SOURCE_COMMIT records the commit the fixtures came from — informational
# (and skips a redundant pull), not a pin: we track latest `main`.

set -euo pipefail

OPS_REPO="https://github.com/aricma/open-productivity-standard.git"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FIXTURES_DIR="$ROOT/tests/fixtures/ops"
SOURCE_COMMIT="$FIXTURES_DIR/SOURCE_COMMIT"

main_sha() {
    git ls-remote "$OPS_REPO" refs/heads/main | awk '{print $1}'
}

update() {
    local current remote
    current="$(cat "$SOURCE_COMMIT")"
    remote="$(main_sha)"
    if [ "$current" = "$remote" ]; then
        echo "OPS fixtures already up to date ($remote)"
        return 0
    fi
    echo "updating from $current to $remote"
    local tmp
    tmp="$(mktemp -d)"
    curl -fsSL "https://codeload.github.com/aricma/open-productivity-standard/tar.gz/refs/heads/main" \
        | tar xz -C "$tmp" --strip-components=1
    rm -rf "$FIXTURES_DIR/examples" "$FIXTURES_DIR/tests"
    cp -r "$tmp/examples" "$FIXTURES_DIR/examples"
    cp -r "$tmp/tests" "$FIXTURES_DIR/tests"
    rm -rf "$tmp"
    printf '%s' "$remote" > "$SOURCE_COMMIT"
    echo "fixtures updated to $remote"
}

case "${1:-}" in
    check)
        current="$(cat "$SOURCE_COMMIT")"
        remote="$(main_sha)"
        if [ "$current" = "$remote" ]; then
            echo "OPS fixtures up to date ($current)"
            exit 0
        else
            echo "OPS fixtures stale: local $current, latest main $remote — run: mask fixtures update"
            exit 1
        fi
        ;;
    update)
        update
        ;;
    *)
        echo "usage: update-fixtures.sh check|update" >&2
        exit 2
        ;;
esac
