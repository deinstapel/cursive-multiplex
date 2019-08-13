#! /bin/sh

set -e

die() {
    printf "\e[31:1mError: %s\e[0m\n" "$1" >&2
    exit 1
}

if [ -z "$GITHUB_USERNAME" ]
then
    die "the GITHUB_USERNAME environment variable is not set"
fi

if [ -z "$GITHUB_TOKEN" ]
then
    die "the GITHUB_TOKEN environment variable is not set"
fi

if [ -z "$GITHUB_REPO_SLUG" ]
then
    die "the GITHUB_REPO_SLUG environment variable is not set"
fi

(
    cd "$(git rev-parse --show-toplevel)/target/shields" || die "cannot find project root!"
    repo="https://${GITHUB_USERNAME}:${GITHUB_PASSWORD}@github.com/${GITHUB_REPO_SLUG}.git"
    tmp_dir=$(mktemp -d -t cursive-multiplex-deploy-XXXXXXXX)

    try=0
    while :; do
        git clone --branch gh-pages "$repo" "$tmp_dir"
        cp -ar ./* "$tmp_dir"

        (
            cd "$tmp_dir"
            git add -A
            git commit -m "Travis CI badge deployment"
            git push
        )

        result=$?
        cd -
        rm -rf "$tmp_dir"

        if [ "$result" -eq 0 ] || [ "$try" -ge 5 ]
        then
            break
        fi

        try=$((try + 1))
    done
)
