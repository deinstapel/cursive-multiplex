#! /bin/sh

set -e

die() {
    printf "\e[31:1mError: %s\e[0m" "$1" >&2
    exit 1
}

(
    cd "$(git rev-parse --show-toplevel)/tests" || die "folder not found: tests"

    ./node_modules/.bin/jest --updateSnapshot || die "failed to update snapshots"
)
