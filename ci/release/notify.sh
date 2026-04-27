#!/usr/bin/env bash
# shellcheck shell=bash
# SPDX-License-Identifier: Apache-2.0

# Notify substrait-packaging that a new spec version has been released.
# Failures are non-fatal — the release has already succeeded at this point.

VERSION="${1}"

gh workflow run spec_released.yml \
  --repo substrait-io/substrait-packaging \
  --field substrait_version="${VERSION}" \
  || echo "Warning: failed to trigger release workflow in substrait-packaging"
