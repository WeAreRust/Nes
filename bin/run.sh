#!/bin/sh
set -eu
set -o pipefail

SCRIPT_DIR=$(dirname "$0")
ABSOLUTE_SCRIPT_DIR=$(cd "$SCRIPT_DIR" && pwd)
BASE_DIR=$(dirname "$ABSOLUTE_SCRIPT_DIR")
$(cd $BASE_DIR/nes && cargo build --quiet)
$BASE_DIR/target/debug/nes "$@"
