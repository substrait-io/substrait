#!/usr/bin/env bash
# shellcheck shell=bash
# SPDX-License-Identifier: Apache-2.0

# Scan every installed pixi environment for known vulnerabilities.
#
# Neither dependabot nor Renovate covers conda advisories -- GHSA and OSV have
# no conda ecosystem -- so the installed environment has to be scanned directly.
# Syft builds an SBOM from the conda metadata that pixi wrote into the prefix,
# and Grype matches it against its vulnerability database.
#
# Output is markdown on stdout so it can be appended to a CI job summary, and
# still reads fine in a terminal. The SBOM and the report for each environment
# are left in ${SBOM_DIR} for triage.
#
# Usage: pixi install --all && pixi run scan
#
# Environment:
#   SBOM_DIR  where to write the SBOMs and reports (default: sbom)
#   FAIL_ON   lowest severity that makes this script exit non-zero (negligible,
#             low, medium, high or critical). Unset by default: report findings
#             without failing.

set -euo pipefail

cd "$(git rev-parse --show-toplevel)" || exit 1

sbom_dir="${SBOM_DIR:-sbom}"
fail_on="${FAIL_ON:-}"
preview_rows=10

shopt -s nullglob
envs=(.pixi/envs/*/)
if [ "${#envs[@]}" -eq 0 ]; then
  echo "error: no installed environments under .pixi/envs" >&2
  echo "run 'pixi install --all' first" >&2
  exit 1
fi

mkdir -p "${sbom_dir}"

threshold_met=0

for env_dir in "${envs[@]}"; do
  name="$(basename "${env_dir}")"
  sbom="${sbom_dir}/syft-${name}.json"
  report="${sbom_dir}/grype-${name}.txt"

  # The catalogers have to be selected explicitly: conda-meta-cataloger reads
  # the conda-meta records pixi wrote into the prefix, and
  # cargo-auditable-binary-cataloger recovers crate versions from Rust binaries
  # built with cargo-auditable. Syft's own JSON is what Grype consumes -- a
  # CycloneDX conversion drops metadata Grype matches on.
  syft "${env_dir}" \
    --select-catalogers=+conda-meta-cataloger,+cargo-auditable-binary-cataloger \
    --output "syft-json=${sbom}" >&2

  # built per environment and never empty, so `set -u` stays happy on bash 3.2
  grype_args=("${sbom}")
  if [ -n "${fail_on}" ]; then
    grype_args+=(--fail-on "${fail_on}")
  fi

  # conda packages carry CPEs rather than PURLs, so matching goes through
  # Grype's CPE path; .grype.yaml pins that on.
  status=0
  grype "${grype_args[@]}" > "${report}" || status=$?

  case "${status}" in
    0) ;;
    # 2 means the --fail-on threshold was met, which is a result, not an error.
    2) threshold_met=1 ;;
    *)
      echo "error: grype failed on the ${name} environment (exit ${status})" >&2
      exit "${status}"
      ;;
  esac

  findings=0
  if [ -s "${report}" ] && head -n 1 "${report}" | grep -q '^NAME'; then
    findings=$(($(wc -l < "${report}") - 1))
  fi

  echo "## \`${name}\` environment"
  echo

  if [ "${findings}" -eq 0 ]; then
    echo "No known vulnerabilities."
    echo
    continue
  fi

  # Grype orders the table by risk, so the head of it is the part worth reading
  # first. The rest goes in a collapsed block to keep the job summary short.
  if [ "${findings}" -gt "${preview_rows}" ]; then
    echo "Highest risk of ${findings} findings:"
    echo
    echo '```'
    head -n "$((preview_rows + 1))" "${report}"
    echo '```'
    echo
  fi

  echo "<details>"
  echo "<summary>All ${findings} findings</summary>"
  echo
  echo '```'
  cat "${report}"
  echo '```'
  echo
  echo "</details>"
  echo
done

if [ "${threshold_met}" -ne 0 ]; then
  echo "Findings at or above severity \`${fail_on}\` were reported above."
  exit 1
fi
