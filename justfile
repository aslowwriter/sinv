#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

log := "warn"

alias b := build
alias t := test
alias c := check
alias l := lint

export JUST_LOG := log

lint:
    cargo clippy --all --all-targets --all-features -- --deny warnings
    cargo fmt --all -- --check
    typos -w .
    taplo fmt .

# Run tests
test:
    cargo test --all

test-clean: clean test

# Build the project
build:
    cargo build

# Build the project
build-release:
    cargo build --release

cargo-doc:
    cargo doc --no-deps --all-features --workspace --open

book-build:
    mdbook serve docs

book-serve:
    mdbook build docs

# Clean the target directory
clean:
    cargo clean

# Check for errors without building (quick dev check)
check:
    cargo check

newest:
    cargo upgrade --incompatible --recursive
    cargo +nightly update --breaking -Z unstable-options

flamegraph:
    cargo flamegraph --profile bench

base:
    cargo bench --profile bench -- --save-baseline base

compare:
    cargo bench --profile bench -- --baseline base

cov:
    cargo llvm-cov --locked --all-features --open

# bit hacky but this should at least work across shells
# checks if there is a pr open from the current branch and if not opens one for you
# will only happen if lint and test pass and there are not uncommitted changes to tracked files
pr: ci
    gh pr list --head "$(git rev-parse --abbrev-ref HEAD)" --json author --jq ". == []" | grep -q "true"
    git diff-index --quiet HEAD --
    gh pr create --web --fill-first

# Run all quality checks: fmt, lint, check, test
ci:
    just lint
    just check
    just test

download-lkd-objects-curl:
    curl -Lo lkd.inv https://docs.kernel.org/objects.inv

compile-benches:
    cargo build --profile bench

run-benchmark-suggest: compile-benches
    hyperfine -N "sphobjinv suggest lkd.inv watchdog --all" -n "sphobjinv suggest (python)" --reference "target/release/sinv suggest watchdog lkd.inv" --reference-name "sinv suggest (rust)"  --export-json suggest-timing.json --warmup 50 --min-runs 100 --time-unit millisecond

run-benchmark-write: compile-benches
    hyperfine -N "sphobjinv co plain lkd.inv lkd.txt -o" -n "sphobjinv write (python)" --reference "target/release/sinv write lkd.inv lkd.txt -f" --reference-name "sinv write (rust)"  --export-json write-timing.json --warmup 50 --min-runs 100 --time-unit millisecond

render-comparisons:
    uv run render.py write-timing.json assets/write-comparison.webp "Writing the linux kernel inventory file to disk (lower is better)"
    uv run render.py suggest-timing.json assets/suggest-comparison.webp "finind matches for 'watchdog' in the linux kernel inventory file (lower is better)"

benchmark: download-lkd-objects-curl run-benchmark-write run-benchmark-suggest render-comparisons
