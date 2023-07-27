# User-Defined Functions

Substrait supports the creation of custom functions using [simple extensions](../extensions/index.md#simple-extensions), using the facilities described in [scalar functions](scalar_functions.md). The functions defined by Substrait use the same mechanism. The extension files for standard functions can be found [here](https://github.com/substrait-io/substrait/tree/main/extensions).

Here's an example function that doubles its input:

!!! info inline end "Implementation Note"
    This implementation is only defined on 32-bit floats and integers but could be defined on all numbers (and even lists and strings).  The user of the implementation can specify what happens when the resulting value falls outside of the valid range for a 32-bit float (either return NAN or raise an error).

``` yaml
%YAML 1.2
---
scalar_functions:
  -
    name: "double"
    description: "Double the value"
    impls:
      - args:
          - name: x
            value: fp32
        options:
          on_domain_error:
            values: [ NAN, ERROR ]
        return: fp32
      - args:
          - name: x
            value: i32
        options:
          on_domain_error:
            values: [ NAN, ERROR ]
        return: i32
```
