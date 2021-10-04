# Specialized Record Expressions

While most all types of operations could be reduced to functions, in some cases this would be overly simplistic. Instead, it is helpful to construct some other expression constructs. 

These constructs should be focused on different expression types as opposed to something that directly related to syntactic sugar. For example, CAST and EXTRACT or SQL operations that are presented using specialized syntax. However, they can easily modeled using a function paradigm with minimal complexity.

## Literal Expressions

For each data type, it is possible to create a literal value for that data type. The representation depends on the serialization format.



## If Expression

An if value expression is an expression composed of one if clause, zero or more else if clauses and an else clause. In pseudo code, they are envisioned as:

```
if <boolean expression> then <result expression 1>
else if <boolean expression> then <result expression 2> (zero or more times)
else <result expression 3>
```

When an if expression is declared, all return expressions must be the same identical type.

#### Shortcut Behavior

An if expression is expected to logically short-circuit on a positive outcome. This means that a skipped else/elseif expression cannot cause an error. For example, this should not actually throw an error despite the fact that the cast operation should fail.

```
if 'value' = 'value' then 0
else cast('hello' as integer) 
```



## Switch Expression

Switch expression allow a selection of alternate branches based on the value of a given expression. They are an optimized form of a generic if expression where all conditions are equality to the same value. In pseudo-code:

```
switch(value)
<value 1> => <return 1> (1 or more times)
<else> => <return 3>
```

Return values for a switch expression must all be of identical type.

#### Shortcut Behavior

As in if expressions, switch expression evaluation should not be interrupted by "roads not taken".



## Or List Equality Expression

A specialized structure that is often used is a large list of possible values. In SQL, these are typically large IN lists. They can composed of one or more fields. There are two common patterns, single value and multi value. In pseudo code they are represented as:

```
Single Value:
expression, [<value1>, <value2>, ... <valueN>]

Multi Value:
[expressionA, expressionB], [[value1a, value1b], [value2a, value2b].. [valueNa, valueNb]]
```

For single value expressions, these are a compact equivalent of `expression = value1 OR expression = value2 OR .. OR expression = valueN`. When using an expression of this type, two things are required. The types of the test expression and all value expressions that are related must be of the same type. Additionally, a function signature for equality must be available for the expression type used.









