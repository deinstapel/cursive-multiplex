#! /bin/sh

set -e

die() {
    printf "\e[31:1mError: %s\e[0m" "$1" >&2
    exit 1
}

(
    cd "./tests" || die "folder not found: ./tests"

    npm install \
        --no-package-lock \
        --loglevel error || die "failed to install end2end test dependencies"

    # Install latest release tmux for special ubuntu
    cd ".."
    wget https://github.com/tmux/tmux/releases/download/2.9a/tmux-2.9a.tar.gz
    tar -xf tmux-2.9a.tar.gz
    cd "tmux-2.9a" || die "folder not found: ./tmux-2.9a"
    ./configure && make
    cd ..
    rm tmux-2.9a.tar.gz
)
