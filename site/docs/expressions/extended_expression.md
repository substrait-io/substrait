# Extended expression

Extended expression is provided for expression level protocol instead of plan rels. It mainly targets for expression only evaluation, such as those computed in Filter/Project/Aggregation rels. Different from original expression defined in substrait protocol, it requires more information to completely describe the computation context, including input data schema, referred function signatures and output schema. 

Besides, as it will be used seperately with plan rel representation, it need include basic fields like Version.

## Input and output data schema

Similar as `base_schema` defined in [ReadRel](https://github.com/substrait-io/substrait/blob/7f272f13f22cd5f5842baea42bcf7961e6251881/proto/substrait/algebra.proto#L58), the input data schema tells name/type/nullibilty and layout info of input data for target expression evalutation. It also has a field `name` to define name of output data.

## Referred expression

It will has one or more referred expressions in this message and the referred expressions can be [Expression](https://github.com/substrait-io/substrait/blob/7f272f13f22cd5f5842baea42bcf7961e6251881/proto/substrait/algebra.proto) or [AggregateFunction](https://github.com/substrait-io/substrait/blob/7f272f13f22cd5f5842baea42bcf7961e6251881/proto/substrait/algebra.proto#L1170). More types of expression can be added for more scenarios.

For multi expressions, user can translate them following same order as it occurs in original plan rel. But it does NOT require the consume side to handle it strictly in previous order. Only need to make sure columns in final output are organized in same order as defined in extended expression message.

## Function extensions

As in the expression message, functions are used by referring function anchor so the related extensions are needed to determine detailed function signature.
