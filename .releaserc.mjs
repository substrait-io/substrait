// SPDX-License-Identifier: Apache-2.0
//
// semantic-release configuration.
//
// This is an ESM (.mjs) config rather than .releaserc.json because the
// release-notes-generator needs a `writerOpts.transform` function to strip git
// trailers (e.g. `Signed-off-by:`) that the conventional-commits parser folds
// into `BREAKING CHANGE` notes. JSON cannot carry functions.
//
// This config defaults to a dry run as a fail-safe: the publishing plugins
// (@semantic-release/github, @semantic-release/git) and the publish/notify exec
// commands are only included when RELEASE_DRY_RUN=false. ci/release/run.sh opts
// in to a real release that way; every other invocation -- including a typo'd
// or forgotten env var -- stays harmless. This matters because semantic-release
// still runs verifyConditions/prepare/publish/success in dry-run mode (it only
// skips tag and push), so the side-effecting plugins must be omitted entirely,
// not merely guarded by --dry-run.

import conventionalcommits from "conventional-changelog-conventionalcommits";

// Git trailers that should never appear in the changelog or release notes.
const TRAILER_KEYS = [
  "Signed-off-by",
  "Co-authored-by",
  "Co-developed-by",
  "Reviewed-by",
  "Acked-by",
  "Tested-by",
  "Reported-by",
  "Suggested-by",
  "Helped-by",
  "Cc",
];
const TRAILER = new RegExp(`^(?:${TRAILER_KEYS.join("|")}):\\s`, "i");

// The conventional-commits parser ends a BREAKING CHANGE note only at a
// recognized reference (closes #..., fixes #...) or another note keyword, not
// at a git trailer -- so a trailing `Signed-off-by:` gets absorbed into the
// note text. Strip such trailing trailer lines.
const stripTrailers = (text) => {
  if (!text) return text;
  const lines = text.split("\n");
  while (lines.length) {
    const last = lines[lines.length - 1].trim();
    if (last === "" || TRAILER.test(last)) {
      lines.pop();
    } else {
      break;
    }
  }
  return lines.join("\n");
};

const preset = await conventionalcommits();
const presetTransform = preset.writer.transform;

// Dry run unless explicitly disabled (ci/release/run.sh sets RELEASE_DRY_RUN=false).
const dryRun = process.env.RELEASE_DRY_RUN !== "false";

export default {
  branches: ["main"],
  preset: "conventionalcommits",
  dryRun,
  plugins: [
    [
      "@semantic-release/commit-analyzer",
      {
        releaseRules: [{ breaking: true, release: "minor" }],
      },
    ],
    [
      "@semantic-release/release-notes-generator",
      {
        // Only `transform` is overridden; the generator merges this over the
        // preset's writer options, so templates/grouping/sorting are kept.
        writerOpts: {
          transform(commit, context) {
            const out = presetTransform(commit, context);
            if (out && Array.isArray(out.notes)) {
              out.notes = out.notes.map((note) => ({
                ...note,
                text: stripTrailers(note.text),
              }));
            }
            return out;
          },
        },
      },
    ],
    [
      "@semantic-release/changelog",
      {
        changelogTitle: "Release Notes\n---",
        changelogFile: "CHANGELOG.md",
      },
    ],
    [
      "@semantic-release/exec",
      {
        verifyConditionsCmd: "ci/release/verify.sh",
        prepareCmd: "ci/release/prepare.sh",
        // publish/notify must not run during a dry run.
        ...(dryRun
          ? {}
          : {
              publishCmd: "ci/release/publish.sh ${nextRelease.version}",
              successCmd: "ci/release/notify.sh ${nextRelease.version}",
            }),
      },
    ],
    // GitHub releases and the release commit only happen in a real release.
    ...(dryRun
      ? []
      : [
          [
            "@semantic-release/github",
            {
              successComment: false,
            },
          ],
          [
            "@semantic-release/git",
            {
              assets: ["CHANGELOG.md"],
              message: "chore(release): ${nextRelease.version}",
            },
          ],
        ]),
  ],
};
