#!/bin/bash

set -e

if [ ! -f target/release/devstrap ]; then
  echo "Building devstrap..."
  cargo build --release
fi

./target/release/devstrap "$@"