# User Defined Functions

Substrait supports the creation of custom functions using the function signature facility described in [scalar functions](scalar_functions.md). If a user wants to declare their own custom functions, they can do either of the following:

* Expose their function signatures publicly using a public organization id. Public organization ids are between 0 and 2B (exclusive) and are registered with the Substrait repository [here](https://github.com/substrait-io/substrait/blob/main/extensions/organizations.yaml).
* Define one or more private organization ids. Private organization ids are 2B and above.

Public organizations should be automatically mapped by tools, private organizations will have to be manually mapped. Once a organization id is defined and mapped, a plan will validate against the function signatures listed in the extensions table of contents file. [Example file](https://github.com/substrait-io/substrait/blob/main/extensions/toc.yaml).

Once a function signature is defined, user defined functions are treated exactly the same as normal functions within the plan.

