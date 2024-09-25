# Common Fields

Every relation contains a common section containing optional hints and emit behavior.


## Emit

A relation which has a direct emit kind outputs the relation's output without reordering or selection.  A relation that specifies an emit output mapping can output its output columns in any order and may leave output columns out.

???+ info "Relation Output"

    * Relations by default provide as their output the list of all of its input columns plus any generated columns as its output columns.  One notable exception is aggregations which only output new columns.


## Hints

Hints provide information that can improve performance but cannot be used to control the behavior.  Table statistics, runtime constraints, name hints, and saved computations all fall into this category.

???+ info "Hint Design"

    * If a hint is not present or has incorrect data the consumer should be able to arrive at the correct result.


### Saved Computations

Computations can be used to save on data structure to use elsewhere.  For instance, let's say we have a plan with a HashEquiJoin and an AggregateDistinct operation.  The HashEquiJoin could save its hash table as part of saved computation id #1 and the AggregateDistinct could read in computation id #1.

Now let's try a more complicated example.  We have a relation that has constructs two hash tables and we'd like one of them to go to our aggregate relation still but the other to go elsewhere.  We can use the computation number to select which data structure goes where.  For instance computation #1 could be hash table number 1 and computation #2 could be hash table number 2.  The reciving entity just needs to know which of its data structures it needs to put that computation in.  So if it has 5 hash table datastructures the LoadedComputation record needs to point to the number that it intends for that incoming data to go.
