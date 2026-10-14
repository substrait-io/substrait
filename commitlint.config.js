module.exports = {
  extends: ["@commitlint/config-conventional"],
  // Bot PR descriptions are not conventional-commit bodies, so don't lint them.
  ignores: [
    // Dependabot, see https://github.com/dependabot/dependabot-core/issues/5923.
    // Kept while dependabot PRs opened before the switch to renovate are still open.
    (message) => /^Bumps \[.+]\(.+\) from .+ to .+\.$/m.test(message),
    (message) => /^Updates the requirements on .+ to permit the latest version\.$/m.test(message),
    // Renovate
    (message) => /^This PR contains the following updates:$/m.test(message),
  ],
  rules: {
    "body-max-line-length": [0, "always", Infinity],
    "footer-max-line-length": [0, "always", Infinity],
    "body-leading-blank": [0, "always"],
  },
};
