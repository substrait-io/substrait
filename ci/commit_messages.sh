#!/usr/bin/env bash
# shellcheck shell=bash

set -euo pipefail

url="${1}"
page=1

while true; do
  json="$(curl -LsS "${url}?page=${page}")"
  len="$(jq length <<< "$json")"
  if [ "$len" -eq "0" ]; then
    break
  fi
  ((++page))
  jq -rcM '.[].commit.message' <<< "$json"
done
