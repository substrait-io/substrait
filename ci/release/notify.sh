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
  --field substrait_version="v${VERSION}" \
  || echo "Warning: failed to trigger release workflow in substrait-packaging"

# Open an adoption-tracking issue in each downstream implementation repo so
# they know a new spec version is available and can track upgrading to it.
DOWNSTREAM_REPOS=(
  "substrait-io/substrait-java"
  "substrait-io/substrait-python"
  "substrait-io/substrait-go"
  "substrait-io/substrait-rs"
  "substrait-io/duckdb-substrait-extension"
)

ISSUE_TITLE="Update to Substrait v${VERSION}"

# The GitHub release was published moments ago by @semantic-release/github, so
# its body holds the generated release notes. Reuse them verbatim in the issue.
# The release is created via the API before this success hook runs, so it should
# already exist; retry a few times only to absorb brief read-replica lag and
# avoid opening issues with an empty release-notes section.
RELEASE_NOTES=""
for i in $(seq 1 3); do
  if RELEASE_NOTES="$(gh release view "v${VERSION}" --repo substrait-io/substrait --json body --jq .body 2>/dev/null)"; then
    break
  fi
  if [ "${i}" -eq 3 ]; then
    echo "Warning: failed to fetch release notes for v${VERSION}; issues will omit them"
    break
  fi
  sleep 10
done

RELEASE_URL="https://github.com/substrait-io/substrait/releases/tag/v${VERSION}"
if [ -n "${RELEASE_NOTES}" ]; then
  ISSUE_BODY="# Release Notes

${RELEASE_NOTES}

See ${RELEASE_URL}"
else
  # Notes couldn't be fetched; still open the issue so the release is not missed.
  # The release URL is the canonical source of the notes anyway.
  ISSUE_BODY="A new Substrait release is available.

See ${RELEASE_URL}"
fi

for repo in "${DOWNSTREAM_REPOS[@]}"; do
  # Avoid opening a duplicate if the release workflow is re-run for this version.
  if gh issue list --repo "${repo}" --state all --search "${ISSUE_TITLE} in:title" \
      --json title --jq '.[].title' | grep -qxF "${ISSUE_TITLE}"; then
    echo "Issue \"${ISSUE_TITLE}\" already exists in ${repo}; skipping."
    continue
  fi

  gh issue create \
    --repo "${repo}" \
    --title "${ISSUE_TITLE}" \
    --body "${ISSUE_BODY}" \
    || echo "Warning: failed to create release notification issue in ${repo}"
done
