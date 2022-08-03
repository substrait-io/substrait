
arithmetic functions
====================

Table of Contents
=================

* [scalar_functions](#scalar_functions)
	* [add](#add)
	* [subtract](#subtract)
	* [multiply](#multiply)
	* [divide](#divide)
	* [negate](#negate)
	* [modulus](#modulus)
	* [power](#power)
	* [sqrt](#sqrt)
	* [cos](#cos)
	* [sin](#sin)
	* [tan](#tan)
	* [acos](#acos)
	* [asin](#asin)
	* [atan](#atan)
	* [atan2](#atan2)
* [aggregate_functions](#aggregate_functions)
	* [sum](#sum)
	* [avg](#avg)
	* [min](#min)
	* [max](#max)
* [window_functions](#window_functions)
	* [row_number](#row_number)
	* [rank](#rank)
	* [dense_rank](#dense_rank)
	* [percent_rank](#percent_rank)
	* [cume_dist](#cume_dist)
	* [ntile](#ntile)


This document file is generated for [functions_arithmetic.yaml](https://github.com/substrait-io/substrait/tree/main/extensions/functions_arithmetic.yaml)

Updating this document with the latest yaml can be done by running: [generate_function_docs.py](https://github.com/substrait-io/substrait/tree/main/site/docs/functions/generate_function_docs.py)
# scalar_functions

## add


DESCRIPTION:  
Add two values.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. add(i8, i8, opt_enum:name_placeholder): -> i8 </br>   
<br> 1. add(i16, i16, opt_enum:name_placeholder): -> i16 </br>   
<br> 2. add(i32, i32, opt_enum:name_placeholder): -> i32 </br>   
<br> 3. add(i64, i64, opt_enum:name_placeholder): -> i64 </br>   
<br> 4. add(fp32, fp32, opt_enum:name_placeholder): -> fp32 </br>   
<br> 5. add(fp64, fp64, opt_enum:name_placeholder): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>name_placeholder ['SILENT', 'SATURATE', 'ERROR'] </li> 

</details>

## subtract


DESCRIPTION:  
Subtract one value from another.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. subtract(i8, i8, opt_enum:name_placeholder): -> i8 </br>   
<br> 1. subtract(i16, i16, opt_enum:name_placeholder): -> i16 </br>   
<br> 2. subtract(i32, i32, opt_enum:name_placeholder): -> i32 </br>   
<br> 3. subtract(i64, i64, opt_enum:name_placeholder): -> i64 </br>   
<br> 4. subtract(fp32, fp32, opt_enum:name_placeholder): -> fp32 </br>   
<br> 5. subtract(fp64, fp64, opt_enum:name_placeholder): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>name_placeholder ['SILENT', 'SATURATE', 'ERROR'] </li> 

</details>

## multiply


DESCRIPTION:  
Multiply two values.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. multiply(i8, i8, opt_enum:name_placeholder): -> i8 </br>   
<br> 1. multiply(i16, i16, opt_enum:name_placeholder): -> i16 </br>   
<br> 2. multiply(i32, i32, opt_enum:name_placeholder): -> i32 </br>   
<br> 3. multiply(i64, i64, opt_enum:name_placeholder): -> i64 </br>   
<br> 4. multiply(fp32, fp32, opt_enum:name_placeholder): -> fp32 </br>   
<br> 5. multiply(fp64, fp64, opt_enum:name_placeholder): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>name_placeholder ['SILENT', 'SATURATE', 'ERROR'] </li> 

</details>

## divide


DESCRIPTION:  
Divide one value by another. Partial values are truncated.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. divide(i8, i8, opt_enum:name_placeholder): -> i8 </br>   
<br> 1. divide(i16, i16, opt_enum:name_placeholder): -> i16 </br>   
<br> 2. divide(i32, i32, opt_enum:name_placeholder): -> i32 </br>   
<br> 3. divide(i64, i64, opt_enum:name_placeholder): -> i64 </br>   
<br> 4. divide(fp32, fp32, opt_enum:name_placeholder): -> fp32 </br>   
<br> 5. divide(fp64, fp64, opt_enum:name_placeholder): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>name_placeholder ['SILENT', 'SATURATE', 'ERROR'] </li> 

</details>

## negate


DESCRIPTION:  
Negation of the value

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. negate(i8, opt_enum:name_placeholder): -> i8 </br>   
<br> 1. negate(i16, opt_enum:name_placeholder): -> i16 </br>   
<br> 2. negate(i32, opt_enum:name_placeholder): -> i32 </br>   
<br> 3. negate(i64, opt_enum:name_placeholder): -> i64 </br>   
<br> 4. negate(fp32): -> fp32 </br>   
<br> 5. negate(fp64): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>name_placeholder ['SILENT', 'SATURATE', 'ERROR'] </li> 

</details>

## modulus


DESCRIPTION:  
Get the remainder when dividing one value by another.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. modulus(i8, i8): -> i8 </br>   
<br> 1. modulus(i16, i16): -> i16 </br>   
<br> 2. modulus(i32, i32): -> i32 </br>   
<br> 3. modulus(i64, i64): -> i64 </br> 

</details>

## power


DESCRIPTION:  
Take the power with the first value as the base and second as exponent.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. power(i64, i64, opt_enum:name_placeholder): -> i64 </br>   
<br> 1. power(fp32, fp32): -> fp32 </br>   
<br> 2. power(fp64, fp64): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>name_placeholder ['SILENT', 'SATURATE', 'ERROR'] </li> 

</details>

## sqrt


DESCRIPTION:  
Square root of the value

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. sqrt(i64, opt_enum:rounding, opt_enum:on_domain_error): -> fp64 </br>   
<br> 1. sqrt(fp32, opt_enum:rounding, opt_enum:on_domain_error): -> fp32 </br>   
<br> 2. sqrt(fp64, opt_enum:rounding, opt_enum:on_domain_error): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>rounding ['TIE_TO_EVEN', 'TIE_AWAY_FROM_ZERO', 'TRUNCATE', 'CEILING', 'FLOOR'] </li>   
<li>on_domain_error ['NAN', 'ERROR'] </li> 

</details>

## cos


DESCRIPTION:  
Get the cosine of a value in radians.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. cos(fp32, opt_enum:rounding): -> fp64 </br>   
<br> 1. cos(fp64, opt_enum:rounding): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>rounding ['TIE_TO_EVEN', 'TIE_AWAY_FROM_ZERO', 'TRUNCATE', 'CEILING', 'FLOOR'] </li> 

</details>

## sin


DESCRIPTION:  
Get the sine of a value in radians.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. sin(fp32, opt_enum:rounding): -> fp64 </br>   
<br> 1. sin(fp64, opt_enum:rounding): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>rounding ['TIE_TO_EVEN', 'TIE_AWAY_FROM_ZERO', 'TRUNCATE', 'CEILING', 'FLOOR'] </li> 

</details>

## tan


DESCRIPTION:  
Get the tangent of a value in radians.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. tan(fp32, opt_enum:rounding): -> fp64 </br>   
<br> 1. tan(fp64, opt_enum:rounding): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>rounding ['TIE_TO_EVEN', 'TIE_AWAY_FROM_ZERO', 'TRUNCATE', 'CEILING', 'FLOOR'] </li> 

</details>

## acos


DESCRIPTION:  
Get the arccosine of a value in radians.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. acos(fp32, opt_enum:rounding, opt_enum:on_domain_error): -> fp64 </br>   
<br> 1. acos(fp64, opt_enum:rounding, opt_enum:on_domain_error): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>rounding ['TIE_TO_EVEN', 'TIE_AWAY_FROM_ZERO', 'TRUNCATE', 'CEILING', 'FLOOR'] </li>   
<li>on_domain_error ['NAN', 'ERROR'] </li> 

</details>

## asin


DESCRIPTION:  
Get the arcsine of a value in radians.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. asin(fp32, opt_enum:rounding, opt_enum:on_domain_error): -> fp64 </br>   
<br> 1. asin(fp64, opt_enum:rounding, opt_enum:on_domain_error): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>rounding ['TIE_TO_EVEN', 'TIE_AWAY_FROM_ZERO', 'TRUNCATE', 'CEILING', 'FLOOR'] </li>   
<li>on_domain_error ['NAN', 'ERROR'] </li> 

</details>

## atan


DESCRIPTION:  
Get the arctangent of a value in radians.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. atan(fp32, opt_enum:rounding): -> fp64 </br>   
<br> 1. atan(fp64, opt_enum:rounding): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>rounding ['TIE_TO_EVEN', 'TIE_AWAY_FROM_ZERO', 'TRUNCATE', 'CEILING', 'FLOOR'] </li> 

</details>

## atan2


DESCRIPTION:  
Get the arctangent of values given as x/y pairs.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. atan2(fp32, fp32, opt_enum:rounding, opt_enum:on_domain_error): -> fp64 </br>   
<br> 1. atan2(fp64, fp64, opt_enum:rounding, opt_enum:on_domain_error): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>rounding ['TIE_TO_EVEN', 'TIE_AWAY_FROM_ZERO', 'TRUNCATE', 'CEILING', 'FLOOR'] </li>   
<li>on_domain_error ['NAN', 'ERROR'] </li> 

</details>

# aggregate_functions

## sum


DESCRIPTION:  
Sum a set of values. The sum of zero elements yields null.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. sum(i8, opt_enum:name_placeholder): -> i64? </br>   
<br> 1. sum(i16, opt_enum:name_placeholder): -> i64? </br>   
<br> 2. sum(i32, opt_enum:name_placeholder): -> i64? </br>   
<br> 3. sum(i64, opt_enum:name_placeholder): -> i64? </br>   
<br> 4. sum(fp32, opt_enum:name_placeholder): -> fp64? </br>   
<br> 5. sum(fp64, opt_enum:name_placeholder): -> fp64? </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>name_placeholder ['SILENT', 'SATURATE', 'ERROR'] </li> 

</details>

## avg


DESCRIPTION:  
Average a set of values. For integral types, this truncates partial values.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. avg(i8, opt_enum:name_placeholder): -> i8? </br>   
<br> 1. avg(i16, opt_enum:name_placeholder): -> i16? </br>   
<br> 2. avg(i32, opt_enum:name_placeholder): -> i32? </br>   
<br> 3. avg(i64, opt_enum:name_placeholder): -> i64? </br>   
<br> 4. avg(fp32, opt_enum:name_placeholder): -> fp32? </br>   
<br> 5. avg(fp64, opt_enum:name_placeholder): -> fp64? </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>name_placeholder ['SILENT', 'SATURATE', 'ERROR'] </li> 

</details>

## min


DESCRIPTION:  
Min a set of values.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. min(i8): -> i8? </br>   
<br> 1. min(i16): -> i16? </br>   
<br> 2. min(i32): -> i32? </br>   
<br> 3. min(i64): -> i64? </br>   
<br> 4. min(fp32): -> fp32? </br>   
<br> 5. min(fp64): -> fp64? </br> 

</details>

## max


DESCRIPTION:  
Max a set of values.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. max(i8): -> i8? </br>   
<br> 1. max(i16): -> i16? </br>   
<br> 2. max(i32): -> i32? </br>   
<br> 3. max(i64): -> i64? </br>   
<br> 4. max(fp32): -> fp32? </br>   
<br> 5. max(fp64): -> fp64? </br> 

</details>

# window_functions

## row_number


DESCRIPTION:  
the number of the current row within its partition.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. row_number(): -> i64? </br> 

</details>

## rank


DESCRIPTION:  
the rank of the current row, with gaps.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. rank(): -> i64? </br> 

</details>

## dense_rank


DESCRIPTION:  
the rank of the current row, without gaps.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. dense_rank(): -> i64? </br> 

</details>

## percent_rank


DESCRIPTION:  
the relative rank of the current row.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. percent_rank(): -> fp64? </br> 

</details>

## cume_dist


DESCRIPTION:  
the cumulative distribution.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. cume_dist(): -> fp64? </br> 

</details>

## ntile


DESCRIPTION:  
Return an integer ranging from 1 to the argument value,dividing the partition as equally as possible.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. ntile(i32): -> i32? </br>   
<br> 1. ntile(i64): -> i64? </br> 

</details>
