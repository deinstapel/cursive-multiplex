#! /bin/sh

set -e

die() {
    printf "\e[31:1mError: %s\e[0m" "$1" >&2
    exit 1
}

(
    cd "$(git rev-parse --show-toplevel)" || die "cannot find project root"

    # Badges!
    mkdir -p ./target/shields
    if cargo build --all-targets; then
      cat <<EOF > "./target/shields/$RUST_CHAIN-build.json"
{
    "color": "brightgreen",
    "isError": true,
    "label": "$RUST_CHAIN",
    "message": "passing",
    "schemaVersion": 1
}
EOF
    else
      PRV_EXIT=$?
      cat <<EOF > "./target/shields/$RUST_CHAIN-build.json"
{
    "color": "red",
    "isError": true,
    "label": "$RUST_CHAIN",
    "message": "failed",
    "schemaVersion": 1
}
EOF
      exit $PRV_EXIT
    fi

    # only run the tests, do not fail build when a test fails
    cargo test --no-fail-fast || true

    # create badge for `cargo test`
    cargo test --no-fail-fast -- -Z unstable-options --format json | \
        jq -s -f ./scripts/shields-from-tests.jq > ./target/shields/cargo-test.json
)
