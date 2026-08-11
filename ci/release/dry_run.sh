#!/usr/bin/env bash
# shellcheck shell=bash
# SPDX-License-Identifier: Apache-2.0

set -euo pipefail

curdir="$PWD"
worktree="$(mktemp -d)"
branch="$(basename "$worktree")"

git worktree add "$worktree"

function cleanup() {
  cd "$curdir" || exit 1
  git worktree remove "$worktree"
  git worktree prune
  git branch -D "$branch"
}

trap cleanup EXIT ERR

cd "$worktree" || exit 1

export GITHUB_REF="$branch"

# Use the committed .releaserc.mjs so the dry run previews the real changelog
# (including the writerOpts that strip git trailers). The config defaults to a
# dry run -- omitting the GitHub/git/publish/notify plugins, which semantic-release
# would otherwise still invoke in dry-run mode (it only skips tag and push) -- so
# no env var is needed here.
npx --yes \
  -p "semantic-release@24.1.2" \
  -p "@semantic-release/commit-analyzer" \
  -p "@semantic-release/release-notes-generator" \
  -p "@semantic-release/changelog" \
  -p "@semantic-release/exec" \
  -p "conventional-changelog-conventionalcommits@8.0.0" \
  semantic-release \
  --ci false \
  --dry-run \
  --branches "$branch" \
  --repository-url "file://$PWD"
