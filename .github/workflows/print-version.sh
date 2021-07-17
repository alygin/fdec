#!/bin/bash

sdir="$(dirname "$0")"
sed -n 's/^version = \"\(.*\)\"/\1/p' < $sdir/../../Cargo.toml
