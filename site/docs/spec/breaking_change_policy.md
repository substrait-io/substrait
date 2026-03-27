---
title: Breaking Change Policy
---

# Substrait Breaking Change Policy

Breaking changes proposed to the specification should include an explicit migration strategy when they are proposed.

The migration strategy must be implemented in all [active libraries](../community/library_support.md) before the breaking change is implemented.

For the purposes of this policy, deprecations are treated as breaking changes, because they are usually the first step in implementing a breaking change.

## Context

There exists in reality a rich set of query languages, query engines and data formats with an equally rich set of semantics. In the course of developing Substrait, we are constantly learning fun and arcane tidbits about all of these systems as we attempt to craft a model to encapsulate them all.

Our model is not perfect. Over time, we have introduced features into Substrait that have not stood the test of practice. In many cases, our work integrating Substrait with new systems has revealed the limitations of our model. We of course take the opportunity to improve our model when this happens, but it does mean that we accumulate cruft in the specification over time — cruft that we would like to remove.

Substrait is still a pre-1.0 project, so while we allow ourselves to make breaking changes, we want to do so in a controlled manner that allows users to deal with the breakages easily. For this reason, we strive to treat breaking changes as migrations and provide users with clear paths to upgrade their systems.

When making any breaking changes we should assume that users are operating Substrait in a multi-system, multi-language environment where parallel and synchronous deployments are impossible. Breaking changes should be structured so that users can update systems independently and asynchronously.

## Breaking Change Case Studies

### Field Replacement

Changes requiring the introduction of a new field to replace an old one.

#### Potential Strategy

1. Mark old field as deprecated.
2. Update the specification to include both fields.
3. Update producers to emit both the new and old fields.
4. Update consumers to read both new and old fields.
5. Remove old field.

#### Example: URI to URI Migration

- Initial Deprecation [PR](https://github.com/substrait-io/substrait/pull/859)
- substrait-java Migration [PR](https://github.com/substrait-io/substrait-java/pull/522)
- substrait-go Migration [PR](https://github.com/substrait-io/substrait-go/pull/166)
