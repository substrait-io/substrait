# Versioning

As an interface specification, the goal of Substrait is to reach a point where (breaking) changes will never need to happen again, or at least be few and far between.
By analogy, Apache Arrow's in-memory format specification has stayed functionally constant, despite many major library versions being released.
However, we're not there yet.
When we believe that we've reached this point, we will signal this by releasing version 1.0.0.
Until then, we will remain in the 0.x.x version regime.

Despite this, we strive to maintain backward compatibility for both the binary representation and the text representation by means of deprecation.
When a breaking change cannot be reasonably avoided, we may remove previously deprecated fields.
All deprecated fields will be removed for the 1.0.0 release.

Substrait uses [semantic versioning](https://semver.org/) for its version numbers, with the addition that, during 0.x.y, we increment the x digit for every release, whether it contains breaking changes, new features, or fixes.
Releases of the specification always have y set to 0.
The y digit is reserved for [substrait-packaging](https://github.com/substrait-io/substrait-packaging), whose artifacts are versioned in lockstep with the specification, so that a broken artifact can be re-published as 0.x.1, 0.x.2, and so on without implying a change to the specification.
This applies to releases after 0.103.1, which was the last patch release of the specification itself.
The release process is currently automated and makes a new release every week, provided something has changed on the main branch since the previous release.
This release cadence will likely be slowed down as stability increases over time.
[Conventional commits](https://www.conventionalcommits.org/en/v1.0.0-beta.2/) are used to distinguish between breaking changes, new features, and fixes,
and GitHub actions are used to verify that there are indeed no breaking protobuf changes in a commit, unless the commit message states this.
