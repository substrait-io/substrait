#!/usr/bin/env bash
# shellcheck shell=bash

set -euxo pipefail

npx --yes \
  -p semantic-release \
  -p "@semantic-release/commit-analyzer" \
  -p "@semantic-release/release-notes-generator" \
  -p "@semantic-release/changelog" \
  -p "@semantic-release/exec" \
  -p "@semantic-release/git" \
  semantic-release \
  --ci false \
  --dry-run \
  --plugins \
  --analyze-commits "@semantic-release/commit-analyzer" \
  --generate-notes "@semantic-release/release-notes-generator" \
  --verify-conditions "@semantic-release/changelog,@semantic-release/exec,@semantic-release/git" \
  --prepare "@semantic-release/changelog,@semantic-release/exec" \
  --branches "$GITHUB_REF" \
  --repository-url "file://$PWD"
