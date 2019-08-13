#! /bin/sh

set -e

die() {
    printf "\e[31:1mError: %s\e[0m" "$1" >&2
    exit 1
}

(
    cd "$(git rev-parse --show-toplevel)" || die "cannot find project root"

    cargo build --all-targets

    # only run the tests, do not fail build when a test fails
    cargo test --no-fail-fast || true

    # create badge for `cargo test`
    cargo test --no-fail-fast -- -Z unstable-options --format json | \
        jq -s -f ./scripts/shields-from-tests.jq > ./target/shields/cargo-test.json
)
