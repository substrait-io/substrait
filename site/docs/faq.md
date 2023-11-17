---
title: FAQ
---
# Frequently Asked Question

## What is the purpose of the post-join filter field on Join relations?
The post-join filter on the various Join relations is not always equivalent to an explicit Filter relation AFTER the Join.

See the example [here](https://facebookincubator.github.io/velox/develop/joins.html#hash-join-implementation) that highlights how the post-join filter behaves differently than a Filter relation in the case of a left join.
