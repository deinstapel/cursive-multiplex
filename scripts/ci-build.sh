#! /bin/sh

die() {
    printf "\e[31:1mError: %s\e[0m\n" "$1" >&2
    exit 1
}

if [ -z "$RUST_CHAIN" ]
then
    die "RUST_CHAIN environment variable is not set! RUST_CHAIN={stable,nightly}"
fi

(
    cd "$(git rev-parse --show-toplevel)" || die "cannot find project root"

    # Badges!
    mkdir -p ./target/shields
    if cargo "+${RUST_CHAIN}" --color=always  build --all-targets; then
      cat <<EOF > "./target/shields/$RUST_CHAIN-build.json"
{
    "color": "brightgreen",
    "isError": true,
    "label": "$RUST_CHAIN build",
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
    "label": "$RUST_CHAIN build",
    "message": "failed",
    "schemaVersion": 1
}
EOF
      exit $PRV_EXIT
    fi

    cargo "+${RUST_CHAIN}" --color=always test --no-fail-fast
    exitcode=$?

    # create badge for `cargo test`
    cargo "+${RUST_CHAIN}" test --no-fail-fast -- -Z unstable-options --format json | \
        jq -s -f ./scripts/shields-from-tests.jq > ./target/shields/cargo-test.json

    exit $exitcode
)
