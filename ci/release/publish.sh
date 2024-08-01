#!/usr/bin/env bash
# shellcheck shell=bash
# SPDX-License-Identifier: Apache-2.0

set -euo pipefail

cd "$(git rev-parse --show-toplevel)"/proto || exit 1

buf push --tag "v${1}" --tag "$(git rev-parse HEAD)"
