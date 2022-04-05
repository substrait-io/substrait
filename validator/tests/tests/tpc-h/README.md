This directory contains positive tests for the TPC-H queries that Isthmus
didn't break for at the time of writing, modified slightly here and there to
fix things that Isthmus wasn't doing right, being:

 - Aggregations output an extra column according to the spec indicating which
   grouping set was used for a particular row, which the Isthmus plans weren't
   considering.
 - Aggregations with only measures were being emitted by Isthmus as
   aggregations with empty grouping sets rather than no grouping sets.
 - Isthmus was emitting duplicate grouping sets wherever there should only be
   one grouping set.
 - Decimal literals had too many bytes attached to them.

TODO: when function resolution is implemented in the validator, the diagnostic
overrides relating to those not currently working should be removed.
