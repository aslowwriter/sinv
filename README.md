# sinv

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/gh/aslowwriter/sinv/branch/main/graph/badge.svg)](https://codecov.io/gh/aslowwriter/sinv)
[![crates.io](https://img.shields.io/crates/v/sinv)](https://crates.io/crates/sinv)
[![Docs.rs](https://docs.rs/sinv/badge.svg)](https://docs.rs/sinv)


A CLI toolkit for inspecting Sphinx object inventories


(Currently mostly for shits and giggles, but who knows.)

## Dev tools
To develop sinv you'll want to have these tools installed:

- [`just`](https://github.com/casey/just) A command runner to run (and document) workflows we run, including installing dev and publish dependencies
- [`typos-cli`](https://github.com/crate-ci/typos) Fixing typos... not that we make any... but you know, just in case.
- [`taplo-cli`](https://github.com/tamasfe/taplo) Keeping our `.toml` files nice and clean
- [`bacon`](https://github.com/Canop/bacon) A runner that will watch your files and run checks, tests, linting etc. when they change. Very useful while developing

##  Publishing Tools
If you have to publish, or otherwise fiddle with dependencies of sinv you'll want these installed as well:
- [`cargo-semver`](https://github.com/obi1kenobi/cargo-semver-checks) A cargo plugin to check that we haven't accidentally broken our API when we didn't mean to.
- [`cargo-edit`](https://github.com/killercup/cargo-edit) A cargo plugin for managing dependencies, incl updating them.
- [`git-cliff`](https://github.com/orhun/git-cliff) A neat tool to generate our changelog

## Template

This repo was initially setup using [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) and [this template](https://github.com/aslowwriter/rust-template)
