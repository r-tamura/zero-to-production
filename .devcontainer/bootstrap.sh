#!/usr/bin/env bash

# zero2productionの開発環境セットアップスクリプト

set -e

# 3.8.2.1 sqlx-cli
cargo install --version="~0.7" sqlx-cli --no-default-features --features rustls,postgres
