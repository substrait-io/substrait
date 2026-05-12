#!/usr/bin/env bash
# shellcheck shell=bash
# SPDX-License-Identifier: Apache-2.0

# Notify substrait-packaging that a new spec version has been released.
# Failures are non-fatal — the release has already succeeded at this point.

VERSION="${1}"

# Wait for the tag to be visible via the GitHub API before notifying downstream.
# The tag is pushed by semantic-release moments before this script runs, and
# GitHub's ref replication can lag enough for the receiving workflow to miss it.
echo "Waiting for tag v${VERSION} to be visible on GitHub..."
for i in $(seq 1 4); do
  if gh api "repos/substrait-io/substrait/git/refs/tags/v${VERSION}" &>/dev/null; then
    echo "Tag v${VERSION} confirmed visible."
    break
  fi
  if [ "${i}" -eq 4 ]; then
    echo "Warning: tag v${VERSION} not visible after 120s; proceeding anyway"
  fi
  sleep 30
done

gh workflow run spec_released.yml \
  --repo substrait-io/substrait-packaging \
  --field substrait_version="${VERSION}" \
  || echo "Warning: failed to trigger release workflow in substrait-packaging"
