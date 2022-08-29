Thank you for submitting a PR!

Before you continue, please ensure that your commit messages follow [conventional commit syntax](1). This will save you a rebase later. Substrait uses an automated release process that, among other things, uses the commit messages to build a changelog, so the syntax and format matters!

The title of the PR should also be a valid commit header.

Some examples of proper commit message headers and PR titles:

 - `feat: add feature X`
 - `fix: X in case of Y`
 - `docs: improve documentation for X`

Note the case and grammar conventions.

Furthermore, any commit that imposes a breaking change should end in a paragraph that starts with `BREAKING CHANGE: ...`, where `...` explains what changed. The automated release process uses this to determine how it should bump the version number. Anything that changes the behavior of a plan that was previously legal is considered a breaking change; note that this includes behavior specifications that only exist in Substrait in the form of behavior descriptions on the website or in comments.

Please also note breaking changes in the first comment of the PR, so we're less likely to miss them when we review or merge.

[1]: https://www.conventionalcommits.org/en/v1.0.0/
