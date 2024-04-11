#!/usr/bin/env bash
# shellcheck shell=bash
# SPDX-License-Identifier: Apache-2.0

set -euo pipefail

# build artifacts
buf build
buf generate
