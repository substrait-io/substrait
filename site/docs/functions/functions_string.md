
string functions
================

Table of Contents
=================

* [scalar_functions](#scalar_functions)
	* [concat](#concat)
	* [like](#like)
	* [substring](#substring)
	* [starts_with](#starts_with)
	* [ends_with](#ends_with)
	* [contains](#contains)
	* [strpos](#strpos)
	* [count_substring](#count_substring)
	* [replace](#replace)


This document file is generated for [functions_string.yaml](https://github.com/substrait-io/substrait/tree/main/extensions/functions_string.yaml)

Updating this document with the latest yaml can be done by running: [generate_function_docs.py](https://github.com/substrait-io/substrait/tree/main/site/docs/functions/generate_function_docs.py)
# scalar_functions

## concat


DESCRIPTION:  
Concatenate two strings

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. concat(varchar&ltL1&gt, varchar&ltL2&gt): -> varchar&ltL1 + L2&gt </br>   
<br> 1. concat(string, string): -> string </br> 

</details>

## like


DESCRIPTION:  
Are two strings like each other.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. like(varchar&ltL1&gt, varchar&ltL2&gt): -> BOOLEAN </br>   
<br> 1. like(string, string): -> BOOLEAN </br> 

</details>

## substring


DESCRIPTION:  
Extract a portion of a string from another string.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. substring(varchar&ltL1&gt, i32, i32): -> varchar&ltL1&gt </br>   
<br> 1. substring(string, i32, i32): -> string </br>   
<br> 2. substring(fixedchar&ltl1&gt, i32, i32): -> string </br> 

</details>

## starts_with


DESCRIPTION:  
Whether this string starts with another string.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. starts_with(varchar&ltL1&gt, varchar&ltL2&gt): -> BOOLEAN </br>   
<br> 1. starts_with(varchar&ltL1&gt, string): -> BOOLEAN </br>   
<br> 2. starts_with(varchar&ltL1&gt, fixedchar&ltL2&gt): -> BOOLEAN </br>   
<br> 3. starts_with(string, string): -> BOOLEAN </br>   
<br> 4. starts_with(string, varchar&ltL1&gt): -> BOOLEAN </br>   
<br> 5. starts_with(string, fixedchar&ltL1&gt): -> BOOLEAN </br>   
<br> 6. starts_with(fixedchar&ltL1&gt, fixedchar&ltL2&gt): -> BOOLEAN </br>   
<br> 7. starts_with(fixedchar&ltL1&gt, string): -> BOOLEAN </br>   
<br> 8. starts_with(fixedchar&ltL1&gt, varchar&ltL2&gt): -> BOOLEAN </br> 

</details>

## ends_with


DESCRIPTION:  
Whether this string ends with another string.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. ends_with(varchar&ltL1&gt, varchar&ltL2&gt): -> BOOLEAN </br>   
<br> 1. ends_with(varchar&ltL1&gt, string): -> BOOLEAN </br>   
<br> 2. ends_with(varchar&ltL1&gt, fixedchar&ltL2&gt): -> BOOLEAN </br>   
<br> 3. ends_with(string, string): -> BOOLEAN </br>   
<br> 4. ends_with(string, varchar&ltL1&gt): -> BOOLEAN </br>   
<br> 5. ends_with(string, fixedchar&ltL1&gt): -> BOOLEAN </br>   
<br> 6. ends_with(fixedchar&ltL1&gt, fixedchar&ltL2&gt): -> BOOLEAN </br>   
<br> 7. ends_with(fixedchar&ltL1&gt, string): -> BOOLEAN </br>   
<br> 8. ends_with(fixedchar&ltL1&gt, varchar&ltL2&gt): -> BOOLEAN </br> 

</details>

## contains


DESCRIPTION:  
Whether this string contains another string.

<details><summary>IMPLEMENTATIONS:</summary>
  
<br> 0. contains(varchar&ltL1&gt, varchar&ltL2&gt): -> BOOLEAN </br>   
<br> 1. contains(varchar&ltL1&gt, string): -> BOOLEAN </br>   
<br> 2. contains(varchar&ltL1&gt, fixedchar&ltL2&gt): -> BOOLEAN </br>   
<br> 3. contains(string, string): -> BOOLEAN </br>   
<br> 4. contains(string, varchar&ltL1&gt): -> BOOLEAN </br>   
<br> 5. contains(string, fixedchar&ltL1&gt): -> BOOLEAN </br>   
<br> 6. contains(fixedchar&ltL1&gt, fixedchar&ltL2&gt): -> BOOLEAN </br>   
<br> 7. contains(fixedchar&ltL1&gt, string): -> BOOLEAN </br>   
<br> 8. contains(fixedchar&ltL1&gt, varchar&ltL2&gt): -> BOOLEAN </br> 

</details>

## strpos


DESCRIPTION:  
Return the position of the first occurrence of a string in another string. The first character of the string is at position 1. If no occurrence is found, 0 is returned.

<details><summary>IMPLEMENTATIONS:</summary>
  
strpos(input, substring): -> `return_type`   
<li>input: The input string.</li>  
<li>substring: The substring to search for.</li>  
<br> 0. strpos(string, string): -> i64 </br>   
<br> 1. strpos(varchar&ltL1&gt, varchar&ltL1&gt): -> i64 </br>   
<br> 2. strpos(fixedchar&ltL1&gt, fixedchar&ltL2&gt): -> i64 </br> 

</details>

## count_substring


DESCRIPTION:  
Return the number of non-overlapping occurrences of a substring in an input string.

<details><summary>IMPLEMENTATIONS:</summary>
  
count_substring(input, substring): -> `return_type`   
<li>input: The input string.</li>  
<li>substring: The substring to count.</li>  
<br> 0. count_substring(string, string): -> i64 </br>   
<br> 1. count_substring(varchar&ltL1&gt, varchar&ltL2&gt): -> i64 </br>   
<br> 2. count_substring(fixedchar&ltL1&gt, fixedchar&ltL2&gt): -> i64 </br> 

</details>

## replace


DESCRIPTION:  
Replace all occurrences of the substring with the replacement string.

<details><summary>IMPLEMENTATIONS:</summary>
  
replace(input, substring, replacement): -> `return_type`   
<li>input: Input string.</li>  
<li>substring: The substring to replace.</li>  
<li>replacement: The replacement string.</li>  
<br> 0. replace(string, string, string): -> string </br>   
<br> 1. replace(varchar&ltL1&gt, varchar&ltL2&gt, varchar&ltL3&gt): -> varchar&ltL1&gt </br> 

</details>
