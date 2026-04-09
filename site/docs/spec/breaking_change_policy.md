---
title: Breaking Change Policy
---

# Substrait Breaking Change Policy

Breaking changes proposed to the specification should include an explicit migration strategy when they are proposed. This is based on the assumption that users are operating Substrait in a multi-system, multi-language environment where parallel and synchronous deployments are impossible. Breaking changes should be made so that users can update systems independently and asynchronously.

The migration strategy must be implemented in all [active libraries](../community/active_libraries.md) before the breaking change is implemented.

## Breaking Change Cookbooks

### URI to URN Migration

In 2025, the Substrait community noticed that the values used for `uris` were not consistent within the ecosystem.

To deal with this, a new URN format was [introduced](https://github.com/substrait-io/substrait/issues/856) to replace URIs. To migrate the ecosystem to use URNs, producers were updated to dual write URIs and URNs in plans, and consumers were updated to preferentially consume URNs when available.

This work can be seen in:
- `substrait-java` Migration [PR](https://github.com/substrait-io/substrait-java/pull/522)
- `substrait-go` Migration [PR](https://github.com/substrait-io/substrait-go/pull/166)
- `substrait-python` Migration [PR](https://github.com/substrait-io/substrait-python/pull/114)

Once all libraries could read and write the new URN format, and had been available for a reasonable duration, the old URI format was fully removed.
