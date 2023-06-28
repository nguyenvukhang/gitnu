#!/usr/bin/env bash

. utils.sh

export CARGO_MANIFEST_DIR=/Users/khang/repos/gitnu

get_cargo_toml_version
echo $RESULT

parse_semver 0.0.3

next_version $RESULT
echo $RESULT
set_cargo_toml_version $RESULT
