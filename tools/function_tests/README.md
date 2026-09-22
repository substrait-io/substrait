# Function test parser

This package contains the reference parser for Substrait function test files.
It turns the ANTLR parse tree for the grammar in `../../grammar/` into the
Python model objects used by coverage checks and extension validation.

The grammar defines the concrete syntax. The visitor in this package also owns
format-level validation that is easier to express after parsing, such as
checking semantic consistency across different parts of a file. Code that loads
`.test` files should use `case_file_parser.py` rather than invoking the ANTLR
parser directly, so those validations run consistently.

Coverage code under `../../tests/coverage/` consumes the parsed test model. It
should only contain coverage-specific checks, not the definition of the function
test file format.
