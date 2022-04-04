#!/usr/bin/env nix-shell
#!nix-shell -p buf -i bash
# shellcheck shell=bash

set -euo pipefail

# build artifacts
buf build
