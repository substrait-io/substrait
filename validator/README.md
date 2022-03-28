Validator
=========

This directory contains a validator for Substrait plans. It's written in Rust,
but bindings are available for Python and C. Other languages may use the C API
via their respective foreign function interface systems.

Command-line interface
----------------------

The easiest way to play around with the validator is via the command-line
interface provided by the Python `substrait-validator` module. At the time of
writing, the package is not yet available on PyPI, but it should be easy enough
to build from source (see the `py` subdirectory). After installing, you should
be able to run:

```console
user@host:~$ substrait-validator
Usage: substrait-validator [OPTIONS] INFILE
Try 'substrait-validator --help' for help.

Error: Missing argument 'INFILE'.
```

If that doesn't work, try `python3 -m substrait-validator`.

Without any options, the validator will decode the given input file based on
the format implied by the file extension, validate the plan, print any
diagnostics encountered, and fail with code 1 if the validator determines that
the plan is invalid. Here's a valid YAML plan as a starting point for playing
around with it:

```yaml
relations:
- rel:
    read:
      namedTable:
        names:
        - person
      baseSchema:
        names:
        - name
        struct:
          nullability: NULLABILITY_REQUIRED
          types:
          - string:
              nullability: NULLABILITY_REQUIRED
```

When you save that as a `.yaml` file and pass it to the validator, it will
simply exit with code 0 without printing anything. Of course, it's more
interesting to try a plan that *isn't* valid, but we'll leave that as an
excercise to the reader.

It's also more interesting to have the validator tell you how it interpreted
the plan. Let's change the command line to do that:

```console
user@host:~$ substrait-validator input.yaml --out-file output.html --mode ignore
```

This generates `output.html`, a self-contained HTML file describing the plan.

Just like the input file, the output file format is derived from the file
extension, so the `.html` part is significant. If you don't want to rely on
this, you can also just specify the formats you want manually using `--in-type`
and `--out-type`.

`--mode ignore` tells the validator to emit a file and exit with code 0
regardless of the validation result. The full list of modes is:

 - `strict`: fail unless the plan was proven to be valid;
 - `loose` (default): fail if the plan was proven to be invalid;
 - `ignore`: ignore the validation result, though the plan still needs some
   level of sanity to succeed; for example, the file must exist, and must
   decode according to the specified file format.
 - `convert`: don't run validation at all; simply convert between different
   representations of the given `substrait.Plan` message. For example, you
   can use this to convert between the binary protobuf serialization format
   and any of the text-based formats supported by the validator.

Note that, without `--mode convert`, the output message type will be
`subtrait.validator.ParseResult` rather than `substrait.Plan` if you use any
of the protobuf-like serialization formats. This message type is a meta
description of the incoming `substrait.Plan` message, with all the information
gathered by the validator annotated to the nodes. The HTML format is pretty
much just a pretty-printed version of this format. More information about this
type is available in the associated `.proto` file.

TODO: diagnostics. Need to expose function to obtain a list of 'em.

For more information, use the `--help` option.

Library usage
-------------

For library usage information, refer to the readme files for the language that
you want to use the library from.