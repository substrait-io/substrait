# Common options

Some (optional) enumeration arguments in the core extensions represent very broad implementation differences and thus appear very frequently. These options are described centrally here for brevity.

## Integer overflow

This optional enumeration appears on all integer functions that can overflow.

| Option      | Description |
|-------------|-------------|
| SILENT      | Overflow silently based on two's complement overflow, i.e. by replacing all higher-order bits that cannot be represented with zero. |
| SATURATE    | Overflow silently by saturating to the maximum or minimum value that can be represented. |
| ERROR       | Throw an error when overflow is detected, aborting the whole query. |
| unspecified | The consumer can decide. If multiple modes are supported, SILENT > SATURATE > ERROR. |

## Floating point (IEEE 754) rounding mode

The specification for floating point numbers defines five different types of rounding modes, to be used when the exact value returned by a computation cannot be exactly represented. Note that this generally does NOT mean rounding to an integer! However, for some functions, this rounding mode is generalized to functions returning integers or returning some fractional index into a list of non-numeric but orderable types.

| Option             | Description |
|--------------------|-------------|
| TIE_TO_EVEN        | Round to the nearest available representation. If the exact value lies exactly between the two nearest representations, tie to even. Note that even-ness for floating point numbers is defined based on the least-significant bit of the mantissa, not on integer even-ness. This is the default behavior of the majority of floating point implementations. |
| TIE_AWAY_FROM_ZERO | Round to the nearest available representation. If the exact value lies exactly between the two nearest representations, tie away from zero. |
| TRUNCATE           | Round to the nearest available representation that lies between zero and the exact value. |
| CEILING            | Round to the nearest available representation that lies between the exact value and positive infinity. |
| FLOOR              | Round to the nearest available representation that lies between negative infinity and the exact value. |
| unspecified        | The consumer can decide. It is recommended that producers use this, as consumers are unlikely to implement more than one mode. |

## Floating point domain error handling

The specification for floating point numbers defines that operations that yield a mathematical domain error may return NaN (not a number) or raise a floating point exception.

| Option      | Description |
|-------------|-------------|
| NAN         | Return NaN when a domain error occurs. |
| ERROR       | Throw an error when a domain error occurs, aborting the whole query. |
| unspecified | The consumer can decide. If it can do both, it should return NaN. |

## Allowable optimizations for statistical functions

Computation of statistical functions is commonly approximated by testing only a subset of the complete population.

| Option      | Description |
|-------------|-------------|
| SAMPLE      | The consumer may choose to operate only on a representative subset of the data. It is up to the consumer to decide which algorithm to use for this, or what the error tolerance is. |
| POPULATION  | The consumer must consider every member of the population when computing the statistical metric. |
| unspecified | The consumer can decide. If it can optimize, it should do so. |

For some functions, a more generalized enumeration is used, that may allow for more optimizations at the cost of accuracy.

| Option      | Description |
|-------------|-------------|
| EXACT       | The consumer must compute the exact value of the metric for the population or sample thereof. |
| APPROXIMATE | The consumer may approximate the metric beyond merely operating on a subset of the data. It is up to the consumer to decide which algorithm to use, or what the error tolerance is. |

## Case sensitivity and conversion

Functions that match strings can generally be configured to match case-sensitive or case-insensitively. In the latter case, they may choose to only match ASCII characters case-insensitively, as this can be more performant than using a complete Unicode lookup table, and may be good enough.

| Option                 | Description |
|------------------------|-------------|
| CASE_SENSITIVE         | Strings must be matched case-sensitively. |
| CASE_INSENSITIVE       | Strings must be matched case-insensitively, using Unicode case conversion rules. |
| CASE_INSENSITIVE_ASCII | Strings that only use ASCII characters must be matched case-insensitively. Non-ASCII characters are not expected. If a non-ASCII character appears nonetheless, the consumer may decide whether to match it case-sensitively or case-insensitively. |
| unspecified            | All strings are expected to use the same case convention, so case sensitivity is not expected to affect the result. Thus, the consumer can decide. It should prefer case-sensitive matching if supported. |

Case conversion functions have a similar option.

| Option      | Description |
|-------------|-------------|
| UTF8        | Case conversion must be done using the complete ruleset defined by Unicode. |
| ASCII       | The consumer should only case-convert ASCII characters. |
| unspecified | Strings that only use ASCII characters must be converted as specified. Non-ASCII characters are not expected. If a non-ASCII character appears nonetheless, the consumer may decide whether to convert its case or leave its case unchanged. |
