#!/usr/bin/env bash
# shellcheck shell=bash

set -euo pipefail

if [[ $GITHUB_REF == refs/tags/v* ]]; then
  buf push --tag "${GITHUB_REF_NAME}"
else
  buf push
fi
