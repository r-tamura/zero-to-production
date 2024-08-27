#!/usr/bin/env bash

# zero2productionの開発環境セットアップスクリプト

set -e
sudo apt-get update
sudo apt-get install -y lld clang libssl-dev postgresql-client

# 3.8.2.1 sqlx-cli
cargo install --version="~0.7" sqlx-cli --no-default-features --features rustls,postgres

# 4.5 Structed Logging
cargo install cargo-udeps --locked
cargo install bunyan
