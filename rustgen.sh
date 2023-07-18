#!/bin/bash

set -euo pipefail

docker run --rm -u "$UID" -v "${PWD}:/local" openapitools/openapi-generator-cli:v7.0.0-beta generate -i /local/schema.json -g rust -o /local/out/rust

pushd out/rust
cargo build
