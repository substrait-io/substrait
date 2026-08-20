---
title: Breaking Change Policy
---

# Substrait Breaking Change Policy

Proposals for breaking changes to the specification should either reference an applicable migration strategy below or include an explicit migration strategy. This is based on the assumption that users are operating Substrait in a multi-system, multi-language environment where parallel and synchronous deployments are impossible. Breaking changes should be made so that users can update systems independently and asynchronously.

The migration strategy must be implemented in all [active libraries](../community/active_libraries.md) before the breaking change is implemented.

## Breaking Change Cookbooks

### Replacing a Protobuf Field

Replacing a protobuf field requires a staged migration so that producers and consumers can be updated independently. Do not use a `oneof` solely for migration because it prevents producers from writing both representations and may change field-presence semantics. Use a `oneof` only when mutual exclusivity is part of the final schema and the proposal explains why the standard migration is insufficient.

1. Add the replacement using a new field number and deprecate the old field.
2. Update consumers to accept both representations. When the replacement is present, consumers must use it and ignore the deprecated field.
3. Update producers to write the replacement. Producers must also write the deprecated field whenever the replacement has an exact representation in the old field, and both fields must be semantically equivalent. When no equivalent old representation exists, producers write only the replacement. This preserves compatibility with older consumers whenever possible without limiting the replacement's new capabilities.
4. After all active libraries consume the replacement and no longer produce the deprecated field, allow a reasonable migration period before removing the deprecated field. Removal is a breaking change, and the removed field's number and name must be reserved.

Documentation for an individual field should identify its replacement and describe any field-specific presence or equivalence rules.

#### URI to URN Migration Example

In [2025](https://github.com/substrait-io/substrait/issues/828), the Substrait community noticed that the values used for `uris` were not consistent within the ecosystem.

To deal with this, a new URN format was [introduced](https://github.com/substrait-io/substrait/issues/856) to replace URIs. To migrate the ecosystem to use URNs, producers were updated to dual write URIs and URNs in plans, and consumers were updated to preferentially consume URNs when available.

This work can be seen in:
- `substrait-java` Migration [PR](https://github.com/substrait-io/substrait-java/pull/522)
- `substrait-go` Migration [PR](https://github.com/substrait-io/substrait-go/pull/166)
- `substrait-python` Migration [PR](https://github.com/substrait-io/substrait-python/pull/114)

Once all libraries could read and write the new URN format, and had been available for a reasonable duration, the old URI format was fully removed.
