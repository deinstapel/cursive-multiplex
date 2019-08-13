#! /bin/sh

set -e

die() {
    printf "\e[31:1mError: %s\e[0m" "$1" >&2
    exit 1
}

(
    cd "$(git rev-parse --show-toplevel)/tests" || die "folder not found: tests"

    npm install \
        --no-package-lock \
        --loglevel error || die "failed to install end2end test dependencies"
)
