#!/bin/bash

sdir="$(dirname "$0")"
ver="$(sed -n 's/^version = \"\(.*\)\"/\1/p' < $sdir/../../Cargo.toml)"
echo ::set-output name=version::$ver