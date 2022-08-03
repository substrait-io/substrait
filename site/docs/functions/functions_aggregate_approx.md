
approx functions
================

Table of Contents
=================

* [aggregate_functions](#aggregate_functions)
	* [approx_count_distinct](#approx_count_distinct)


This document file is generated for [functions_aggregate_approx.yaml](https://github.com/substrait-io/substrait/tree/main/extensions/functions_aggregate_approx.yaml)

Updating this document with the latest yaml can be done by running: [generate_function_docs.py](https://github.com/substrait-io/substrait/tree/main/site/docs/functions/generate_function_docs.py)
# aggregate_functions

## approx_count_distinct


DESCRIPTION:  
Calculates the approximate number of rows that contain distinct values of the expression argument using HyperLogLog. This function provides an alternative to the COUNT (DISTINCT expression) function, which returns the exact number of rows that contain distinct values of an expression. APPROX_COUNT_DISTINCT processes large amounts of data significantly faster than COUNT, with negligible deviation from the exact result.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. approx_count_distinct(any): -> i64 </br> 

</details>
