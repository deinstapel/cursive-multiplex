#! /bin/sh

set -e

die() {
    printf "\e[31:1mError: %s\e[0m" "$1" >&2
    exit 1
}

(
    cd "./tests" || die "folder not found: ./tests"

    ./node_modules/.bin/jest --updateSnapshot || die "failed to update snapshots"
)
