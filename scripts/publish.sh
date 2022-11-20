#!/usr/bin/env bash

SEMVER=""

if [ ! -z "$(git status --porcelain)" ]; then
  printf "\e[1;33mWARNING: there are uncommitted changes.\e[1;0m\n"
fi

TAG=""
TAG=$(git tag | tail -n 1)

cargo build --release

while read -r LINE; do
  if [[ $LINE =~ ^version.*\"(.*)\" ]]; then
    SEMVER=${BASH_REMATCH[1]}
    if [[ $SEMVER =~ ([0-9]+)\.([0-9]+)\.([0-9]+) ]]; then
      echo "Cargo.toml:  v$SEMVER"
      echo "current tag: $TAG"
      if [[ "v$SEMVER" != "$TAG" ]]; then
        echo "Current version and current tag don't match."
        echo "Fix this before continuing."
        exit 1
      fi
    else
      echo "Invalid version number"
      exit 1
    fi
  fi
done < <(cat $CARGO_MANIFEST_DIR/Cargo.toml)

publish() {
  # final check
  cargo test || exit 1
  cargo publish || exit 1
  git push origin $TAG
}

printf "Continuing will:
  1) push tags to remote
  2) publish to cargo registry
"
printf "confirm? (Y/n) "
read -n 1 CONFIRM
printf '\n'
if [[ $CONFIRM != 'Y' ]] && [[ $CONFIRM != 'y' ]]; then
  printf "Aborted.\n"
  exit 0
fi
publish
