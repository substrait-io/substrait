
datetime functions
==================

Table of Contents
=================

* [scalar_functions](#scalar_functions)
	* [extract](#extract)
	* [add](#add)
	* [add_intervals](#add_intervals)
	* [subtract](#subtract)
	* [lte](#lte)
	* [lt](#lt)
	* [gte](#gte)
	* [gt](#gt)


This document file is generated for [functions_datetime.yaml](https://github.com/substrait-io/substrait/tree/main/extensions/functions_datetime.yaml)

Updating this document with the latest yaml can be done by running: [generate_function_docs.py](https://github.com/substrait-io/substrait/tree/main/site/docs/functions/generate_function_docs.py)
# scalar_functions

## extract


DESCRIPTION:  
Extract portion of a date/time value.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. extract(timestamp, req_enum:The part of the value to extract.): -> i64 </br>   
<br> 1. extract(timestamp_tz, req_enum:The part of the value to extract.): -> i64 </br>   
<br> 2. extract(date, req_enum:The part of the value to extract.): -> i64 </br>   
<br> 3. extract(time, req_enum:The part of the value to extract.): -> i64 </br> 

</details>


<details><summary>OPTIONS:</summary>
  
<li>The part of the value to extract. ['YEAR', 'MONTH', 'DAY', 'SECOND'] </li>   
<li>The part of the value to extract. ['YEAR', 'MONTH', 'DAY'] </li>   
<li>The part of the value to extract. ['SECOND'] </li> 

</details>

## add


DESCRIPTION:  
Add an interval to a date/time type.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. add(timestamp, interval_year): -> timestamp </br>   
<br> 1. add(timestamp_tz, interval_year): -> timestamp </br>   
<br> 2. add(date, interval_year): -> timestamp </br>   
<br> 3. add(timestamp, interval_day): -> timestamp </br>   
<br> 4. add(timestamp_tz, interval_day): -> timestamp </br>   
<br> 5. add(date, interval_day): -> timestamp </br> 

</details>

## add_intervals


DESCRIPTION:  
Add two intervals together.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. add_intervals(interval_day, interval_day): -> interval_day </br>   
<br> 1. add_intervals(interval_year, interval_year): -> interval_year </br> 

</details>

## subtract


DESCRIPTION:  
Subtract an interval from a date/time type.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. subtract(timestamp, interval_year): -> timestamp </br>   
<br> 1. subtract(timestamp_tz, interval_year): -> timestamp_tz </br>   
<br> 2. subtract(date, interval_year): -> date </br>   
<br> 3. subtract(timestamp, interval_day): -> timestamp </br>   
<br> 4. subtract(timestamp_tz, interval_day): -> timestamp_tz </br>   
<br> 5. subtract(date, interval_day): -> date </br> 

</details>

## lte


DESCRIPTION:  
less than or equal to

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. lte(timestamp, timestamp): -> boolean </br>   
<br> 1. lte(timestamp_tz, timestamp_tz): -> boolean </br>   
<br> 2. lte(date, date): -> boolean </br>   
<br> 3. lte(interval_day, interval_day): -> boolean </br>   
<br> 4. lte(interval_year, interval_year): -> boolean </br> 

</details>

## lt


DESCRIPTION:  
less than

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. lt(timestamp, timestamp): -> boolean </br>   
<br> 1. lt(timestamp_tz, timestamp_tz): -> boolean </br>   
<br> 2. lt(date, date): -> boolean </br>   
<br> 3. lt(interval_day, interval_day): -> boolean </br>   
<br> 4. lt(interval_year, interval_year): -> boolean </br> 

</details>

## gte


DESCRIPTION:  
less than or equal to

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. gte(timestamp, timestamp): -> boolean </br>   
<br> 1. gte(timestamp_tz, timestamp_tz): -> boolean </br>   
<br> 2. gte(date, date): -> boolean </br>   
<br> 3. gte(interval_day, interval_day): -> boolean </br>   
<br> 4. gte(interval_year, interval_year): -> boolean </br> 

</details>

## gt


DESCRIPTION:  
less than

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. gt(timestamp, timestamp): -> boolean </br>   
<br> 1. gt(timestamp_tz, timestamp_tz): -> boolean </br>   
<br> 2. gt(date, date): -> boolean </br>   
<br> 3. gt(interval_day, interval_day): -> boolean </br>   
<br> 4. gt(interval_year, interval_year): -> boolean </br> 

</details>
