# Extended expression

Extended expression is a top-level message, like a plan, but for expressions. They can hold expressions to be used in filter, projection, and aggregation operations that aren't part of a full plan. Because it is a top-level message, it includes more context than expressions that exist _within_ a plan. Some of this is additional computational context, such as the input data schema and the output schema.  In addition there is message metadata, such as the Substrait version and the URIs of referenced extensions.

For details, see the definition in [extended_expression.proto](https://github.com/substrait-io/substrait/blob/main/proto/substrait/extended_expression.proto).

## Referred expression

Extended expressions can contain one or multiple expressions. For example, a single expression could be used for filter operations, while multiple expressions could be used used for projections and aggregations. Each of these expressions is represented by an `ExpressionReference` message, which can either contain an [Expression](https://github.com/substrait-io/substrait/blob/7f272f13f22cd5f5842baea42bcf7961e6251881/proto/substrait/algebra.proto) or [AggregateFunction](https://github.com/substrait-io/substrait/blob/7f272f13f22cd5f5842baea42bcf7961e6251881/proto/substrait/algebra.proto#L1170) message. In the future, other types might be allowed to handle different use cases.

If multiple expressions are specified then there is no implicit relationship between the different top-level expressions. Each expression should be considered independent and operates on the same top-level schema. This means that the expressions cannot build upon one another. For example,  the expressions `x + 1 as y` and `y - 2` cannot be represented in the same message, since the second expression refers to the first (assuming the field `y` was not in the input schema).

## Input and output data schema

The schema of the input data is specified by the `base_schema` field.

The schema of the output data is determined by each of the `ExpressionReference` messages in the `referred_expr` field. The output type for the expression is determined by the expression or measure. The field name is determined by `output_names`. For primitive types `output_names` is just one string, but if the output type is a struct it will contain a name for each field, in depth-first order.

## Order

If there are multiple expressions, the field order of the input and output data matters. Input data will match the schema order
in `base_schema` and output data must match the order of the `ExpressionReference` messages. However, there is no requirement that the expressions are executed in any particular order.

## Function extensions

Just like expressions in plans, functions are referred to with a function anchor. So similar to plans, the function declarations must be included in `extensions` and the URIs of the extension YAML files in `extension_uris`.

