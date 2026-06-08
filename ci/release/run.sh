#!/usr/bin/env bash
# shellcheck shell=bash
# SPDX-License-Identifier: Apache-2.0

set -euo pipefail

# .releaserc.mjs defaults to a dry run; opt in to a real release here.
RELEASE_DRY_RUN=false npx --yes \
  -p "semantic-release@24.1.2" \
  -p "@semantic-release/commit-analyzer" \
  -p "@semantic-release/release-notes-generator" \
  -p "@semantic-release/changelog" \
  -p "@semantic-release/github" \
  -p "@semantic-release/exec" \
  -p "@semantic-release/git" \
  -p "conventional-changelog-conventionalcommits@8.0.0" \
  semantic-release --ci
