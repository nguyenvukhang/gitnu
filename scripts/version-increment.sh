#!/usr/bin/env bash

# 1. update Cargo.toml (version)
# 2. update Cargo.lock (package version)
# 3. commit all this and tag it with [$NEXT_VERSION]

DIRNAME=$(dirname $(readlink -f $0))

. $DIRNAME/utils.sh

get_latest_git_tag
GIT_TAG_VERSION=$RESULT

get_cargo_toml_version
CARGO_TOML_VERSION=$RESULT

if [[ "$CARGO_TOML_VERSION" != "$GIT_TAG_VERSION" ]]; then
  echo "Cargo.toml version and current tag don't match."
  echo "Cargo.toml: $CARGO_TOML_VERSION"
  echo "Git tag:    $GIT_TAG_VERSION"
  echo "Fix this before continuing."
  exit 1
fi

next_version $GIT_TAG_VERSION
NEXT_VERSION="$RESULT"

set_cargo_toml_version $NEXT_VERSION

# at this point, Cargo.toml has been updated.
# next: create a build to update Cargo.lock, create a new commit,
# and tag that commit.

# this mutates actual state
mutate() {
  mv $TMP $CARGO_MANIFEST_DIR/Cargo.toml
  cargo build --release # to update Cargo.toml

  git add $CARGO_MANIFEST_DIR/Cargo.toml $CARGO_MANIFEST_DIR/Cargo.lock
  git commit -m "ver: bump to $NEXT_VERSION"
  git tag $NEXT_VERSION
}

# vim:ft=sh
