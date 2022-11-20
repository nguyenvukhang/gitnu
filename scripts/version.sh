#!/usr/bin/env bash

SEMVER=""
TMP=Cargo.tmp

if [ ! -z "$(git status --porcelain)" ]; then
  printf "\e[1;33mWARNING: there are uncommitted changes.\e[1;0m\n"
  exit 1
fi

TAG="v0.0.0"
TAG=$(git tag | tail -n 1)

cleanup() {
  rm -f $TMP
}
trap cleanup EXIT

rm -f $TMP
while read -r LINE; do
  if [[ $LINE =~ ^version.*\"(.*)\" ]]; then
    SEMVER=${BASH_REMATCH[1]}
    if [[ $SEMVER =~ ([0-9]+)\.([0-9]+)\.([0-9]+) ]]; then
      echo "current version: v$SEMVER"
      echo "current tag:     $TAG"
      if [[ "v$SEMVER" != "$TAG" ]]; then
        echo "Current version and current tag don't match."
        echo "Fix this before continuing."
        exit 1
      fi
      major=${BASH_REMATCH[1]}
      minor=${BASH_REMATCH[2]}
      patch=${BASH_REMATCH[3]}
      let patch++
      NEXTVER="$major.$minor.$patch"
      echo "version = \"$NEXTVER\"" >>$TMP
      NEXTVER="v$NEXTVER"
      printf "next version:    \e[1;32m$NEXTVER\e[1;0m\n"
    else
      echo "Invalid version number"
      exit 1
    fi
  else
    echo $LINE >>$TMP
  fi
done < <(cat $CARGO_MANIFEST_DIR/Cargo.toml)

[ -z $NEXTVER ] && exit 0

# at this point, Cargo.toml has been updated.
# next: create a build to update Cargo.lock, create a new commit,
# and tag that commit.

# this mutates actual state
mutate() {
  mv $TMP $CARGO_MANIFEST_DIR/Cargo.toml
  cargo build --release # to update Cargo.toml
  git add $CARGO_MANIFEST_DIR/Cargo.toml $CARGO_MANIFEST_DIR/Cargo.lock
  git commit -m "ver: bump to $NEXTVER"
  git tag $NEXTVER
}

printf "Continuing will:
  1) update Cargo.toml (version)
  2) update Cargo.lock (package version)
  3) commit all this and tag it with [$NEXTVER]
"
printf "confirm? (Y/n) "
read -n 1 CONFIRM
printf '\n'
if [[ $CONFIRM != 'Y' ]] && [[ $CONFIRM != 'y' ]]; then
  printf "Aborted.\n"
  exit 0
fi
mutate

# vim:ft=sh
