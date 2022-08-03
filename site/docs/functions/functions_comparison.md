
comparison functions
====================

Table of Contents
=================

* [scalar_functions](#scalar_functions)
	* [not_equal](#not_equal)
	* [equal](#equal)
	* [is_not_distinct_from](#is_not_distinct_from)
	* [lt](#lt)
	* [gt](#gt)
	* [lte](#lte)
	* [gte](#gte)
	* [is_null](#is_null)
	* [is_not_null](#is_not_null)
	* [is_nan](#is_nan)


This document file is generated for [functions_comparison.yaml](https://github.com/substrait-io/substrait/tree/main/extensions/functions_comparison.yaml)

Updating this document with the latest yaml can be done by running: [generate_function_docs.py](https://github.com/substrait-io/substrait/tree/main/site/docs/functions/generate_function_docs.py)
# scalar_functions

## not_equal


DESCRIPTION:  
Whether two values are not_equal (nulls are considered not not-equal).

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. not_equal(any1, any1): -> BOOLEAN </br> 

</details>

## equal


DESCRIPTION:  
Whether two values are equal (nulls are considered unequal).

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. equal(any1, any1): -> BOOLEAN </br> 

</details>

## is_not_distinct_from


DESCRIPTION:  
Whether two values are equal (nulls are considered equal).

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. is_not_distinct_from(any1, any1): -> BOOLEAN </br> 

</details>

## lt


DESCRIPTION:  
Less than

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. lt(any1, any1): -> BOOLEAN </br> 

</details>

## gt


DESCRIPTION:  
Greater than

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. gt(any1, any1): -> BOOLEAN </br> 

</details>

## lte


DESCRIPTION:  
Less than or equal to

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. lte(any1, any1): -> BOOLEAN </br> 

</details>

## gte


DESCRIPTION:  
Greater than or equal to

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. gte(any1, any1): -> BOOLEAN </br> 

</details>

## is_null


DESCRIPTION:  
Whether a value is null.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. is_null(any1): -> BOOLEAN </br> 

</details>

## is_not_null


DESCRIPTION:  
Whether a value is not null.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. is_not_null(any1): -> BOOLEAN </br> 

</details>

## is_nan


DESCRIPTION:  
Whether a value is not a number.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. is_nan(fp32): -> BOOLEAN </br>   
<br> 1. is_nan(fp64): -> BOOLEAN </br> 

</details>
