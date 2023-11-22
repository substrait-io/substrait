# Extended Expression

Extended Expression messages are provided for expression-level protocols as an alternative to using a Plan. They mainly target expression-only evaluations, such as those computed in Filter/Project/Aggregation rels. Unlike the original Expression defined in the substrait protocol, Extended Expression messages require more information to completely describe the computation context including: input data schema, referred function signatures, and output schema. 

Since Extended Expression will be used seperately from the Plan rel representation, it will need to include basic fields like Version.

=== "ExtendedExpression Message"

    ```proto
%%% proto.message.ExtendedExpression %%%
    ```

## Input and output data schema

Similar to `base_schema` defined in [ReadRel](https://github.com/substrait-io/substrait/blob/7f272f13f22cd5f5842baea42bcf7961e6251881/proto/substrait/algebra.proto#L58), the input data schema describes the name/type/nullibilty and layout info of input data for the target expression evalutation. It also has a field `name` to define the name of the output data.

## Referred expression

An Extended Exression will have one or more referred expressions, which can be either [Expression](https://github.com/substrait-io/substrait/blob/7f272f13f22cd5f5842baea42bcf7961e6251881/proto/substrait/algebra.proto) or [AggregateFunction](https://github.com/substrait-io/substrait/blob/7f272f13f22cd5f5842baea42bcf7961e6251881/proto/substrait/algebra.proto#L1170). Additional types of expressions may be added in the future.

For a message with multiple expressions, users may produce each Extended Expression in the same order as they occur in the original Plan rel. But, the consumer does NOT have to handle them in this order. A consumer needs only to ensure that the columns in the final output are organized in the same order as defined in the message.

## Function extensions

Function extensions work the same for both Extended Expression and the original Expression defined in the Substrait protocol.
