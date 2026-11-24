#!/usr/bin/env bash
# shellcheck shell=bash
# SPDX-License-Identifier: Apache-2.0

# Refuse to release any version with a non-zero patch component. Substrait
# releases only ever bump the minor version; patch versions are reserved for
# substrait-packaging to re-publish fixed artifacts for an existing spec version.

set -euo pipefail

VERSION="${1}"

if [[ ! "${VERSION}" =~ ^[0-9]+\.[0-9]+\.0$ ]]; then
  echo "Refusing to release ${VERSION}: Substrait releases must have patch version 0." >&2
  echo "Patch versions are reserved for substrait-packaging re-releases." >&2
  exit 1
fi
