#!/usr/bin/env bash
# shellcheck shell=bash
# SPDX-License-Identifier: Apache-2.0

# Render the pixi.lock changes between a base revision and the working tree as
# markdown on stdout.
#
# pixi.lock is marked `-diff` in .gitattributes, so neither git nor the GitHub
# UI shows a textual diff for it. This makes the change reviewable instead.
#
# Usage: pixi run lock-diff [<base-revision>]   (default: main)

set -euo pipefail

base_rev="${1:-main}"

cd "$(git rev-parse --show-toplevel)" || exit 1

before="$(mktemp)"
trap 'rm -f "${before}"' EXIT

if ! git show "${base_rev}:pixi.lock" > "${before}"; then
  echo "error: no pixi.lock at revision '${base_rev}'" >&2
  exit 1
fi

# --manifest-path lets pixi-diff tell explicit dependencies from transitive
# ones, so the handful of lines a human declared are listed separately from the
# hundreds that follow from them.
pixi-diff \
  --before "${before}" \
  --after pixi.lock \
  --manifest-path pyproject.toml \
  | pixi-diff-to-markdown --hide-tables auto --max-expanded-rows 20
