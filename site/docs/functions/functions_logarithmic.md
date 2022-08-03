
logarithmic functions
=====================

Table of Contents
=================

* [scalar_functions](#scalar_functions)
	* [ln](#ln)
	* [log10](#log10)
	* [log2](#log2)
	* [logb](#logb)


This document file is generated for [functions_logarithmic.yaml](https://github.com/substrait-io/substrait/tree/main/extensions/functions_logarithmic.yaml)

Updating this document with the latest yaml can be done by running: [generate_function_docs.py](https://github.com/substrait-io/substrait/tree/main/site/docs/functions/generate_function_docs.py)
# scalar_functions

## ln


DESCRIPTION:  
Natural logarithm of the value

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. ln(fp32, opt_enum:rounding, opt_enum:on_domain_error): -> fp32 </br>   
<br> 1. ln(fp64, opt_enum:rounding, opt_enum:on_domain_error): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>rounding ['TIE_TO_EVEN', 'TIE_AWAY_FROM_ZERO', 'TRUNCATE', 'CEILING', 'FLOOR'] </li>   
<li>on_domain_error ['NAN', 'ERROR'] </li> 

</details>

## log10


DESCRIPTION:  
Logarithm to base 10 of the value

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. log10(fp32, opt_enum:rounding, opt_enum:on_domain_error): -> fp32 </br>   
<br> 1. log10(fp64, opt_enum:rounding, opt_enum:on_domain_error): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>rounding ['TIE_TO_EVEN', 'TIE_AWAY_FROM_ZERO', 'TRUNCATE', 'CEILING', 'FLOOR'] </li>   
<li>on_domain_error ['NAN', 'ERROR'] </li> 

</details>

## log2


DESCRIPTION:  
Logarithm to base 2 of the value

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. log2(fp32, opt_enum:rounding, opt_enum:on_domain_error): -> fp32 </br>   
<br> 1. log2(fp64, opt_enum:rounding, opt_enum:on_domain_error): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>rounding ['TIE_TO_EVEN', 'TIE_AWAY_FROM_ZERO', 'TRUNCATE', 'CEILING', 'FLOOR'] </li>   
<li>on_domain_error ['NAN', 'ERROR'] </li> 

</details>

## logb


DESCRIPTION:  
Logarithm of the value with the given base
logb(x, b) => log_{b} (x)


<details><summary>IMPLEMENTATIONS:</summary>
  
logb(x, base, opt_enum:rounding, opt_enum:on_domain_error): -> `return_type`   
<li>x: The number `x` to compute the logarithm of</li>  
<li>base: The logarithm base `b` to use</li>  
<br> 0. logb(fp32, fp32, opt_enum:rounding, opt_enum:on_domain_error): -> fp32 </br>   
<br> 1. logb(fp64, fp64, opt_enum:rounding, opt_enum:on_domain_error): -> fp64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>rounding ['TIE_TO_EVEN', 'TIE_AWAY_FROM_ZERO', 'TRUNCATE', 'CEILING', 'FLOOR'] </li>   
<li>on_domain_error ['NAN', 'ERROR'] </li> 

</details>
