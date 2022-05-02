#!/usr/bin/env bash
set -eo pipefail

if ! bunyan --help > /dev/null; then
    echo >&2 "Error: cargo bunyan is not installed. Install it with 'cargo install bunyan'"
    exit 1
fi

# We are using the `bunyan` CLI to prettify the outputted logs
# The original `bunyan` requires NPM, but you can install a Rust-port with
# `cargo install bunyan`
TEST_LOG=true cargo test health_check_works | bunyan
