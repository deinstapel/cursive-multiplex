#! /bin/sh

die() {
    printf "\e[31:1mError: %s\e[0m\n" "$1" >&2
    exit 1
}

if [ -z "$GITHUB_ACTOR" ]
then
    die "the GITHUB_ACTOR environment variable is not set"
fi

if [ -z "$GITHUB_TOKEN" ]
then
    die "the GITHUB_TOKEN environment variable is not set"
fi

if [ -z "$GITHUB_REPOSITORY" ]
then
    die "the GITHUB_REPOSITORY environment variable is not set"
fi

(
    cd "$(git rev-parse --show-toplevel)/target/shields" || die "cannot find project root!"
    repo="https://${GITHUB_ACTOR}:${GITHUB_TOKEN}@github.com/${GITHUB_REPOSITORY}.git"
    tmp_dir=$(mktemp -d -t cursive-multiplex-deploy-XXXXXXXX)

    git config --global user.email "runner@ci"
    git config --global user.name "Github CI Runner"
    try=0
    while :; do
        if ! git clone --branch gh-pages "$repo" "$tmp_dir"
        then
            (
                cd "$tmp_dir" || die "failed to enter temporary directory"
                git init
                git remote add origin "$repo"
                git checkout -b gh-pages
            )
        fi

        cp -ar ./* "$tmp_dir"

        (
            cd "$tmp_dir" || die "failed to enter temporary directory"
            git add -A
            git commit -m "Github CI badge deployment"
            git push origin gh-pages:gh-pages
        )

        result=$?
        if [ "$result" -eq 0 ] || [ "$try" -ge 5 ]
        then
            break
        fi

        try=$((try + 1))
    done

    rm -rf "$tmp_dir"

)
