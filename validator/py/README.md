# Python bindings for validator

This directory contains a Rust/PyO3 project to generate Python bindings for the
validator library.

## Installation

No wheels are published yet at this time. To build manually, you will need:

 - [rust](https://www.rust-lang.org/tools/install)
 - [maturin](https://github.com/PyO3/maturin)

At which point you can run:

```console
user@host:/path/to/substrait/validator/py$ maturin build
...
📦 Built wheel for CPython 3.10 to /path/to/substrait/target/wheels/substrait_validator-...whl
user@host:/path/to/substrait/validator/py$ pip install --force-reinstall /path/to/substrait/target/wheels/substrait_validator-...whl
```

Note: copy the wheel path from `maturin`'s output.

If you prefer to work from a venv, you should also be able to just run `maturin develop`.

## Usage

Currently, the API looks something like this:

```python
>>> from substrait_validator import ParseTree
>>> tree = ParseTree(b'binary repr of substrait.Plan message goes here')
>>> tree.check_valid()
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
ValueError: Error (plan): failed to parse proto format
```

It should by no means be considered to be stable though, at this point. Run

```python
>>> from substrait_validator import ParseTree
>>> help(ParseTree)
```

for up-to-date usage information.
