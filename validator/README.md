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
Missing input file. Try --help for usage information.
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

For more information, use the `--help` option.

Library usage
-------------

For library usage information, refer to the readme files for the language that
you want to use the library from.

Diagnostics
-----------

The primary output of the validator (beyond its validity verdict) is a list of
diagnostics. In fact, the validator derives its verdict from this list. Each
diagnostic consists of the following bits of information:

 - a severity, being either info, warning, or error;
 - a classification, represented using a 4-digit diagnostic code;
 - a cause description; and
 - a path into the protobuf/YAML tree, pointing to the node that the diagnostic
   originated from.

The severity levels strictly map as follows:

 - an error means that something is invalid;
 - a warning means that something may or may not be invalid (i.e. validity
   could not be determined for some reason); and
 - info has no effect on validity.

Once the validator as gathered all diagnostics, the validity of the plan is
simply determined by the above mapping applied to the highest severity level
encountered.

Note that the command line interface specifically could be said to have an
extra "fatal" level. Such fatal diagnostics are not strictly diagnostics in the
sense that they are validation results; they simply indicate that the CLI
returned a non-zero exit code and why.

Severity levels can be clamped to a certain range, distinguished by their
classification. This allows you to, for example, disable warnings of a certain
type by clamping them down to info when you know that those particular warnings
are not of interest to your application, or raise severity to error if you want
the validator to be extra pedantic about something. Because the validator
derives its verdict from the highest-severity diagnostic encountered, clamping
severity levels may also change the verdict.

You can request the list of diagnostic codes from the command-line interface
using the `--help-diagnostics` flag:

```console
user@host:~$ substrait-validator --help-diagnostics
The following diagnostic codes are defined:

0000 (Unclassified): unclassified diagnostic.
 |- 0001 (NotYetImplemented): not yet implemented.
 |- 0002 (IllegalValue): illegal value.
...
```

Diagnostic codes are organized in a tree. When you configure the severity range
of a diagnostic code with children, its children will inherit this
configuration, unless they themselves are also explicitly configured. For
example, you can disable all warnings and errors except for those corresponding
to one particular diagnostic by clamping code 0000 down to info only, and then
overriding the configuration for the diagnostic you're interested in back to
the full info to error range.
