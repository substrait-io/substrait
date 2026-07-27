# Common Fields

Every relation contains a common section containing optional hints and emit behavior.


## Emit

A relation which has a direct emit kind outputs the relation's output without reordering or selection.  A relation that specifies an emit output mapping can output its output columns in any order and may leave output columns out.

An unset `emit_kind` is equivalent to `Direct`: the relation outputs its columns as is, without reordering or selection.  This is also the behavior when the relation carries no `common` section at all.  Producers therefore only need to set `emit_kind` explicitly when they want an `Emit` remapping; `Direct` and an unset `emit_kind` are semantically identical.

???+ info "Relation Output"

    * Many relations (such as Project) by default provide as their output the list of all their input columns plus any generated columns as its output columns.  Review each relation to understand its specific output default.


## Relation ID

A relation may carry an optional plan-wide unique identifier (`rel_anchor`). When set, the value must be >= 1 and unique across all relations in the plan. This identifier is required when the relation is the binding point for an `OuterReference` that uses `rel_reference` resolution. See [Field References — Outer References](../expressions/field_references.md#outer-references) for details.

## Hints

Hints provide information that can improve performance but cannot be used to control the behavior.  Table statistics, runtime constraints, name hints, and saved computations all fall into this category.

???+ info "Hint Design"

    * If a hint is not present or has incorrect data the consumer should be able to ignore it and still arrive at the correct result.


### Saved Computations

Computations can be used to save a data structure to use elsewhere.  For instance, let's say we have a plan with a HashEquiJoin and an AggregateDistinct operation.  The HashEquiJoin could save its hash table as part of saved computation id number 1 and the AggregateDistinct could read in computation id number 1.
