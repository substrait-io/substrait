
generic functions
=================

Table of Contents
=================

* [aggregate_functions](#aggregate_functions)
	* [count](#count)
	* [count](#count)


This document file is generated for [functions_aggregate_generic.yaml](https://github.com/substrait-io/substrait/tree/main/extensions/functions_aggregate_generic.yaml)

Updating this document with the latest yaml can be done by running: [generate_function_docs.py](https://github.com/substrait-io/substrait/tree/main/site/docs/functions/generate_function_docs.py)
# aggregate_functions

## count


DESCRIPTION:  
Count a set of values

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. count(any, opt_enum:name_placeholder): -> i64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>name_placeholder ['SILENT', 'SATURATE', 'ERROR'] </li> 

</details>

## count


DESCRIPTION:  
Count a set of records (not field referenced)

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. count(opt_enum:name_placeholder): -> i64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>name_placeholder ['SILENT', 'SATURATE', 'ERROR'] </li> 

</details>
