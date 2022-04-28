#!/usr/bin/env bash
# shellcheck shell=bash

set -euo pipefail

buf push --tag "${1}" --tag "$(git rev-parse HEAD)"
