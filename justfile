alias s := setup
alias b := build
alias t := test
alias tc := test-cov
alias r := run
alias c := check
alias f := fmt
alias l := lint
alias re := render

setup:
    just build
    pre-commit install --install-hooks

build:
    cargo build

test:
    cargo test

test-cov:
    cargo tarpaulin --out Html --target-dir ./target/tarpaulin --skip-clean

run:
    cargo run --bin cooking_book

check:
    pre-commit run -a

fmt:
    cargo fmt

lint:
    cargo clippy

render:
    sass --watch {{ justfile_directory() }}/static/assets/scss/styles.scss {{ justfile_directory() }}/static/assets/css/styles.css
