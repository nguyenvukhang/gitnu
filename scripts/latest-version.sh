#!/usr/bin/env sh

# gets the latest version by looking at git tags

TAG="v0.0.0"
TAG=$(git tag | tail -n 1)
echo $TAG
