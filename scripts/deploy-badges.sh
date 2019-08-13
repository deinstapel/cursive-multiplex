#! /bin/sh

set -e

die() {
    printf "\e[31:1mError: %s\e[0m\n" "$1" >&2
    exit 1
}

(
    cd "$(git rev-parse --show-toplevel)/target/shields" || die "cannot find project root!"
    repo=$(git remote get-url origin)
    tmp_dir=$(mktemp -d -t cursive-multiplex-deploy-XXXXXXXX)

    try=0
    while :; do
        git clone --branch gh-pages "$repo" "$tmp_dir"
        cp -ar ./* "$tmp_dir"

        (
            cd "$tmp_dir"
            git add -A
            git commit -m "Travis CI badge deployment"
            cat <<EOF | git push
$GITHUB_USERNAME
$GITHUB_TOKEN
EOF
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
