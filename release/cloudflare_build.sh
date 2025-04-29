#!/bin/bash
# This script is run by Cloudflare Pages for deployment

set -e
set -x

# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build the app
cd web
npm ci
npm run wasm-release
npm run build --if-present
