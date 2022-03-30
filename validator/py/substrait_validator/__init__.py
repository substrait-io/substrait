# SPDX-License-Identifier: Apache-2.0

import sys
import json
import jsom
import yaml
import click
import urllib.request
from io import BytesIO
from typing import Iterable
from google.protobuf import json_format
from google.protobuf.message import DecodeError as ProtoDecodeError
from .substrait_validator import ResultHandle, Config as _Config, get_diagnostic_codes
from .substrait.plan_pb2 import Plan
from .substrait.validator.validator_pb2 import ParseResult, Diagnostic, Path


class _JsomFormatter:
    def __init__(self, indent_str='  ', length_limit=80, value_limit=5):
        """
        Creates a JSOM formatter using the given configuration.
         - indent_str specifies which indentation character(s) to use.
         - length_limit specifies an approximate maximum number of characters
           that will be considered for emitting a group of tokens on a single
           line.
         - value_limit specifies the number of contained non-nested values
           that will be considered for emitting a group of tokens as a single
           line. Nested tokens will always only be inlined if there are no
           other values in the group.
        """
        self._indent_str = indent_str
        self._length_limit = length_limit
        self._value_limit = value_limit

    @staticmethod
    def _is_open(token):
        """Whether a token is some kind of open-parenthesis."""
        return isinstance(token, str) and token in set('({[<')

    @staticmethod
    def _is_close(token):
        """Whether a token is some kind of close-parenthesis."""
        return isinstance(token, str) and token in set(')}]>')

    @staticmethod
    def _is_key(token):
        """Whether a token is an object key."""
        return isinstance(token, str) and token.startswith('.')

    @staticmethod
    def _is_at(token):
        """Whether a token is an @ selection."""
        return isinstance(token, str) and token.startswith('@')

    @staticmethod
    def _is_spacing(token):
        """Whether a token is merely spacing."""
        return isinstance(token, str) and not token.strip()

    @staticmethod
    def _is_comment(token):
        """Whether a token is a comment."""
        return isinstance(token, str) and token.startswith('#')

    @staticmethod
    def _is_nested(token):
        """Whether a "token" is actually a list of tokens, used to aid
        formatting heuristics."""
        return not isinstance(token, str)

    @classmethod
    def _preprocess(cls, tokens) -> list:
        """
        Preprocess the given iterable of tokens:
         - Combine parenthesized groups of tokens into nested lists, such
           that open-paren tokens can only appear at the start of a list and
           close-parens can only appear at the end (unless they don't close
           anything that's currently open, in which case they're treated as a
           value token later on).
         - Remove any pre-existing whitespace (there probably won't be any if
           this is run on the output of the encoder).
        """
        root = []
        stack = [root]
        for token in tokens:

            # Strip whitespace.
            token = token.strip()
            if not token:
                continue

            # @ commands break out of everything, back to the root.
            if cls._is_at(token):
                del stack[1:]

            # Open-paren tokens create a new nested token list, starting with their
            # open-paren token and (unless interrupted by an @) ending with their
            # close-paren token.
            elif cls._is_open(token):
                sub_tokens = []
                stack[-1].append(sub_tokens)
                stack.append(sub_tokens)

            # Push the token to the current innermost list.
            stack[-1].append(token)

            # Close-paren tokens close the current list level.
            if cls._is_close(token) and len(stack) > 1:
                stack.pop()

        return root

    def _should_wrap(self, tokens: list) -> bool:
        """Whether the given list of nested tokens should be wrapped. This is
        a completely heuristic thing."""

        # Accumulate the approximate line length (not counting indentation)
        # we'd get if we don't wrap and the number of values in the token list.
        # When either reaches the limit, we decide to wrap.
        length = 0
        values = 0

        # Never wrap if there are no tokens.
        if not tokens:
            return False

        # Accumulate the length of the tokens before this one.
        for token in reversed(self._output):
            if '\n' in token:
                break
            length += len(token)

        # Accumulate length and value count of tokens
        for token in tokens:

            # Groups containing @ commands or comments must always be wrapped.
            if self._is_at(token) or self._is_comment(token):
                return True

            # Update value count. Nested groups increment the number of values
            # to the limit minus one, so a single nested group on its own is
            # okay, but combined with any amount of values it will be too long.
            if self._is_nested(token):
                values += self._value_limit - 1
            elif not self._is_key(token):
                values += 1
            if values >= self._value_limit:
                return True

            # Update approximate string length. This doesn't include nested
            # groups, but that's okay, because they will either always be
            # either the last token or we're wrapping anyway due to the value
            # count.
            if isinstance(token, str):
                length += len(token)
            length += 1
            if length >= self._length_limit:
                return True

        # Did not reach any of the limits, keep on single line.
        return False

    def _emit_newline(self, count=1):
        """Emit the given amount of newlines with the current indentation
        level. Overrides any previously emitted whitespace."""
        if self._output and self._is_spacing(self._output[-1]):
            self._output.pop()
        self._output.append(count*'\n' + self._indent*self._indent_str)

    def _emit_token(self, token):
        """Emit the given token, followed by the minimum amount of spacing
        needed to ensure that it won't combine with any other token that
        may be next. May change the whitespace between it and the previous
        token to make things look nicer."""

        # Handle non-nested tokens first.
        if not self._is_nested(token):
            self._output.append(token)
            self._output.append(' ')
            return
        tokens = token

        # Only the last token in a list can be a close.
        close_token = None
        if tokens and self._is_close(tokens[-1]):
            *tokens, close_token = tokens

        # Only the first token in a list can be an open.
        open_token = None
        if tokens and self._is_open(tokens[0]):
            open_token, *tokens = tokens

        # Determine whether we should wrap these tokens.
        wrap = self._should_wrap(tokens)

        # Emit open token, if any.
        if open_token is not None:
            self._emit_token(open_token)

        # Update indentation level.
        if wrap and open_token is not None:
            self._indent += 1

            # ( is used for macro invocations, where the first token is the
            # macro. It looks nicer if the macro name is on the same line.
            if tokens[0] != '(':
                self._emit_newline()

        # Handle nested tokens.
        in_comment = False
        for token in tokens:

            # Do a hard break all the way back to indentation level 0 before
            # any @ token.
            if self._is_at(token):
                self._indent = 0
                self._emit_newline(2)

            # Do a double newline before the first line comment in a sequence.
            if not in_comment and self._is_comment(token):
                self._emit_newline(2)

            # Emit the token.
            self._emit_token(token)

            # Always emit a newline after a line comment, since it's necessary
            # to terminate it.
            in_comment = self._is_comment(token)
            if in_comment:
                self._emit_newline()

            # Emit a newline after values if we're wrapping.
            if wrap and not self._is_key(token):
                self._emit_newline()

        # Update indentation level.
        if wrap and close_token is not None:
            if self._indent > 0:
                self._indent -= 1
            self._emit_newline()

        # Emit close token, if any.
        if close_token is not None:
            self._emit_token(close_token)

    def __call__(self, tokens) -> str:
        """Formats the given token stream."""
        self._output = []
        self._indent = 0
        self._emit_token(self._preprocess(tokens))
        self._indent = 0
        self._emit_newline()
        return ''.join(self._output)


_JSOM_MACROS = """@macros
.field .selection { .directReference .structField .field ?v .rootReference {} }
.field0 .selection { .directReference .structField {} .rootReference {} }
.nullable .nullability "NULLABILITY_NULLABLE"
.required .nullability "NULLABILITY_REQUIRED"

@output
"""


def _jsom_coder() -> jsom.JsomCoder:
    coder = jsom.JsomCoder()
    coder.decode(_JSOM_MACROS)
    return coder


def _jsom_dumps(data) -> str:
    return _JSOM_MACROS + _jsom_coder().encode(data, formatter=_JsomFormatter())


def _jsom_loads(data: str):
    return _jsom_coder().decode(data)


def _populate_config(cfg):
    """We can't derive from _Config to add the add_urllib_resolver() function,
    so we'll just have to monkey-patch it."""
    def generate_method(cls, name, fn):
        def x(self, *args, **kwargs):
            return fn(self._config, *args, **kwargs)
        x.__name__ = name
        x.__doc__ = f.__doc__
        setattr(cls, name, x)

    for name in dir(_Config):
        if name.startswith('_'):
            continue
        f = getattr(_Config, name)
        if not callable(f):
            continue
        generate_method(cfg, name, f)
    cfg.__doc__ = _Config.__doc__
    return cfg


@_populate_config
class Config:
    def __init__(self):
        self._config = _Config()

    @staticmethod
    def _unwrap(config):
        if isinstance(config, Config):
            return config._config
        elif isinstance(config, _Config):
            return config
        elif config is None:
            return None
        else:
            raise TypeError("unsupported type: {}".format(type(config)))

    def add_urllib_resolver(self):
        """Adds a URI resolver based on urllib."""
        def urllib_resolver(uri):
            return urllib.request.urlopen(uri).read()
        self._config.add_uri_resolver(urllib_resolver)


def load_plan_from_proto(data: bytes) -> Plan:
    """Load a Substrait plan from its protobuf serialization."""
    if not isinstance(data, bytes):
        raise TypeError("unsupported type: {}".format(type(data)))
    plan = Plan()
    plan.ParseFromString(data)
    return plan


def load_plan_from_json(data: str) -> Plan:
    """Load a Substrait plan from its JSON string representation."""
    if not isinstance(data, str):
        raise TypeError("unsupported type: {}".format(type(data)))
    return json_format.Parse(data, Plan())


def load_plan_from_dict(data: dict) -> Plan:
    """Load a Substrait plan from its Python object JSON representation."""
    if not isinstance(data, dict):
        raise TypeError("unsupported type: {}".format(type(data)))
    return load_plan_from_json(json.dumps(data))


def load_plan_from_yaml(data: str) -> Plan:
    """Load a Substrait plan from YAML data mimicking the structure of
    its JSON string representation."""
    if not isinstance(data, str):
        raise TypeError("unsupported type: {}".format(type(data)))
    return load_plan_from_dict(yaml.safe_load(data))


def load_plan_from_jsom(data: str) -> Plan:
    """Load a Substrait plan from JSOM data mimicking the structure of
    its JSON string representation."""
    if not isinstance(data, str):
        raise TypeError("unsupported type: {}".format(type(data)))
    return load_plan_from_dict(_jsom_loads(data))


def load_plan(data) -> Plan:
    """Loads a plan from its binary protobuf serialization (bytes input),
    a JSON string (string input), or a dictionary representation of such a
    JSON string (dict input). If data is already a Plan, this function is
    no-op and simply returns its input."""
    if isinstance(data, Plan):
        return data
    elif isinstance(data, bytes):
        return load_plan_from_proto(data)
    elif isinstance(data, str):
        return load_plan_from_json(data)
    elif isinstance(data, dict):
        return load_plan_from_dict(data)
    else:
        raise TypeError("unsupported type: {}".format(type(data)))


def parse_plan(plan, config=None) -> ParseResult:
    """Parses the given plan with the validator. plan can be anything
    supported by load_plan(), a Plan object, or a ResultHandle object. This is
    just an alternate name for plan_to_parse_result()."""
    return plan_to_parse_result(plan, config)


def plan_to_proto(plan) -> bytes:
    """Converts a plan to its binary protobuf serialization. plan can be
    anything supported by load_plan()."""
    return load_plan(plan).SerializeToString()


def plan_to_json(plan) -> str:
    """Converts a plan to its JSON serialization, returned as a string. plan
    can be anything supported by load_plan()."""
    return json_format.MessageToJson(load_plan(plan))


def plan_to_dict(plan) -> dict:
    """Converts a plan to its JSON serialization, returned as a dict. plan can
    be anything supported by load_plan()."""
    return json_format.MessageToDict(load_plan(plan))


def plan_to_yaml(plan) -> str:
    """Converts a plan to the YAML equivalent of its JSON serialization,
    returned as a string. plan can be anything supported by load_plan()."""
    return yaml.safe_dump(plan_to_dict(plan))


def plan_to_jsom(plan) -> str:
    """Converts a plan to the JSOM equivalent of its JSON serialization,
    returned as a string. plan can be anything supported by load_plan()."""
    return _jsom_dumps(plan_to_dict(plan))


def plan_to_result_handle(plan, config=None) -> ResultHandle:
    """Parses a Substrait plan using the validator, and returns its result
    handle object. plan can be anything supported by load_plan(). If the
    input is already a ResultHandle, it is returned as-is."""
    if isinstance(plan, ResultHandle):
        return plan
    if isinstance(plan, bytes):
        data = plan
    else:
        data = plan_to_proto(plan)
    return ResultHandle(data, Config._unwrap(config))


def plan_to_parse_result(plan, config=None) -> ParseResult:
    """Parses the given plan with the validator, and returns its parse result.
    plan can be anything supported by load_plan(), a Plan object, or a
    ResultHandle object."""
    result = ParseResult()
    result.ParseFromString(plan_to_parse_result_proto(plan, config))
    return result


def plan_to_parse_result_proto(plan, config=None) -> str:
    """Same as parse_plan(), but returns the binary serialization of the
    parse result. This is faster, if you don't plan to use the serialization
    from python."""
    return plan_to_result_handle(plan, config).export_proto()


def plan_to_diagnostics(plan, config=None) -> Iterable[Diagnostic]:
    """Converts a plan to an iterable of Diagnostics. plan can be anything
    supported by plan_to_result_handle()."""
    def walk(node):
        for data in node.data:
            if data.HasField('child'):
                for diagnostic in walk(data.child.node):
                    yield diagnostic
            elif data.HasField('diagnostic'):
                yield data.diagnostic
    return walk(plan_to_parse_result(plan, config).root)


def plan_to_diagnostics_str(plan, config=None) -> str:
    """Converts a plan to a multiline string representing the diagnostic
    messages returned by the validator for that plan. plan can be anything
    supported by plan_to_result_handle()."""
    return plan_to_result_handle(plan, config).export_diagnostics()


def plan_to_html(plan, config=None) -> str:
    """Generates a HTML page for the given plan to serve as documentation
    while debugging. plan can be anything supported by
    plan_to_result_handle()."""
    return plan_to_result_handle(plan, config).export_html()


def check_plan(plan, config=None) -> int:
    """Returns 1 if the given plan is valid, -1 if it is invalid, or 0 if the
    validator cannot determine validity. plan can be anything supported by
    load_plan(), a Plan object, or a ResultHandle object."""
    return plan_to_result_handle(plan, config).check()


def check_plan_valid(plan, config=None):
    """Throws a ValueError exception containing the first error or warning
    encountered in the plan if the validator cannot prove correctness of
    the given plan. plan can be anything supported by load_plan(), a Plan
    object, or a ResultHandle object."""
    plan_to_result_handle(plan, config).check_valid()


def check_plan_not_invalid(plan, config=None):
    """Throws a ValueError exception containing the first error encountered in
    the plan if the validator can prove that the given plan is invalid. plan
    can be anything supported by load_plan(), a Plan object, or a ResultHandle
    object."""
    plan_to_result_handle(plan, config).check_not_invalid()


def path_to_string(path: Path) -> str:
    """Converts a substrait.validator.Path message to a string."""
    elements = [path.root]
    for element in path.elements:
        if element.HasField('field'):
            elements.append(f'.{element.field.field}')
        elif element.HasField('repeated_field'):
            elements.append(f'.{element.repeated_field.field}[{element.repeated_field.index}]')
        elif element.HasField('oneof_field'):
            elements.append(f'.{element.oneof_field.field}<{element.oneof_field.variant}>')
        elif element.HasField('array_element'):
            elements.append(f'[{element.array_element.index}]')
        else:
            raise ValueError('invalid path element')
    return ''.join(elements)


@click.command()
@click.argument('infile', required=False)
@click.option('--in-type',
              type=click.Choice(
                  ['ext', 'proto', 'json', 'yaml', 'jsom'],
                  case_sensitive=False),
              default='ext',
              help=('Input file type. "ext" uses the extension of the input '
                    'file, defaulting to "proto" if there is none.'))
@click.option('--verbosity',
              '-v',
              type=click.Choice(
                  ['info', 'warn', 'error', 'fatal', 'quiet'],
                  case_sensitive=False),
              default='warn',
              help=('Specifies the verbosity for writing diagnostics to '
                    'stderr.'))
@click.option('--out-file', '-O',
              default=None,
              help='Output file. "-" may be used to select stdout.')
@click.option('--out-type',
              type=click.Choice(
                  ['ext', 'diag', 'html', 'proto', 'json', 'yaml', 'jsom'],
                  case_sensitive=False),
              default='ext',
              help=('Output file type. "ext" uses the extension of the output '
                    'file, defaulting to "diag" if there is none.'))
@click.option('--mode',
              '-m',
              type=click.Choice(
                  ['convert', 'ignore', 'loose', 'strict'],
                  case_sensitive=False),
              default='loose',
              help=('Validation mode. "convert" disables all but protobuf\'s '
                    'internal validation, and can be used to convert between '
                    'different representations of substrait.Plan. "ignore" '
                    'runs validation, but ignores the result (i.e. the '
                    'program always returns 0 and emits an output file if '
                    'requested). "loose" fails only if the validator can '
                    'prove that the plan is invalid. "strict" fails if it '
                    'cannot prove that it is valid.'))
@click.option('--ignore-unknown-fields',
              help=('Do not generate warnings for unknown protobuf fields '
                    'that are set to their protobuf-defined default value.'))
@click.option('--allow-proto-any',
              multiple=True,
              help=('Explicitly allow the given protobuf type URL(s) to be '
                    'used in protobuf Any messages. Supports glob syntax.'))
@click.option('--diagnostic-level',
              nargs=3,
              multiple=True,
              help=('Clamps the error level of diagnostics with diagnostic '
                    'code or class [0] to at least [1] and at most [2]. '
                    'For example, --diagnostic-level 1 warn error will '
                    'override the level of info diagnostics with code 1 '
                    'to warning, leaving the other levels unchanged.'))
@click.option('--override-uri',
              nargs=2,
              multiple=True,
              help=('Overrides URIs in the plan that match [0] with [1]. Set '
                    '[1] to "-" to disable resolution of matching URIs. '
                    'Supports glob syntax. For example, '
                    '"--override-uri http://* -" disables resolution via '
                    'http.'))
@click.option('--use-urllib/--no-use-urllib',
              default=True,
              help=('Enable URI resolution via urllib. Enabled by default. '
                    'If disabled, only file:// URIs will resolve (after '
                    'application of any --override-uri options).'))
@click.option('--help-diagnostics',
              is_flag=True,
              help=('Show a list of all known diagnostic codes and exit.'))
def cli(infile, in_type, out_file, out_type, mode, verbosity,
        ignore_unknown_fields, allow_proto_any, diagnostic_level,
        override_uri, use_urllib, help_diagnostics):
    """Validate or convert the substrait.Plan represented by INFILE (or stdin
    using "-").

    The following formats are supported:

    \b
     - proto: binary serialization format of protobuf.
     - json: JSON serialization format of protobuf.
     - yaml: like JSON, but represented as YAML.
     - jsom: like JSON, but represented as JSOM (still highly experimental,
       see https://github.com/saulpw/jsom).
     - diag*: list of validator diagnostic messages.
     - html*: all information known about the plan in HTML format.

    *output-only, and not supported in -mconvert mode.

    When validation is enabled, the output message type will be
    substrait.validator.Result. If you just want to convert between different
    representations of the substrait.Plan message, use -mconvert.
    """

    # Define various helper functions and constants.
    INFO = Diagnostic.Level.LEVEL_INFO
    WARN = Diagnostic.Level.LEVEL_WARNING
    ERROR = Diagnostic.Level.LEVEL_ERROR
    FATAL = ERROR + 1
    QUIET = FATAL + 1

    def level_str_to_int(level):
        """Converts a string representation of an error level or verbosity to
        its internal integer representation."""
        return {
            'info': INFO,
            'warn': WARN,
            'error': ERROR,
            'fatal': FATAL,
            'quiet': QUIET,
        }[level]

    def emit_diagnostic(level, msg, code=None, source=None, original_level=None):
        """Emits a diagnostic message to stderr."""

        # Only print the diagnostic if the configured verbosity is high enough.
        if level < verbosity_level:
            return

        # Determine the original error level.
        if original_level is None:
            original_level = level

        # Format the level.
        formatted = [{
            FATAL: click.style('Fatal error', fg='red', bold=True),
            ERROR: click.style('Error', fg='red', bold=True),
            WARN: click.style('Warning', fg='yellow', bold=False),
            INFO: click.style('Info', fg='green', bold=False),
        }[level]]

        # Format extra information written within parentheses.
        parens = []
        if original_level != level:
            if original_level > level:
                mod = 'reduced from '
            else:
                mod ='promoted from '
            mod += {
                FATAL: 'fatal',
                ERROR: 'error',
                WARN: 'warning',
                INFO: 'info',
            }[original_level]
            parens.append(mod)
        if code is not None:
            parens.append(f'code {code:04d}')
        if parens:
            formatted.append(' ({})'.format(', '.join(parens)))
        formatted.append(':\n')

        # Append source information, if known.
        if source is not None:
            formatted.append(f'  at {source}:\n')

        # Append the actual message.
        formatted.append('  ')
        formatted.append('\n  '.join(str(msg).split('\n')))
        formatted.append('\n')

        # Print the formatted diagnostic.
        click.echo(''.join(formatted), err=True)

    def fatal(*args, **kwargs):
        """Shorthand for emit_diagnostic(FATAL, ...) followed by exiting with
        code 1."""
        emit_diagnostic(FATAL, *args, **kwargs)
        sys.exit(1)

    def deduce_format(fil, typ, remap):
        """Deduces the file format for fil with type hint typ using the rules
        in remap."""
        if typ == 'ext':
            if fil is None:
                typ = remap['DEFAULT']
            else:
                _, *ext = fil.rsplit('.', maxsplit=1)
                if ext:
                    typ = ext[0].lower()
                typ = remap.get(typ, remap['DEFAULT'])
        return typ

    def emit_output(data):
        """Emits the given output data as specified on the command line."""
        # Encode text formats as unicode.
        if not isinstance(data, bytes):
            data = data.encode('utf-8')

        # Write to the output.
        if out_file == '-':
            try:
                count = sys.stdout.buffer.write(data)
            except IOError as e:
                fatal(f'failed to write to stdout: {e}')
        elif out_file is not None:
            try:
                with open(out_file, 'wb') as f:
                    count = f.write(data)
            except IOError as e:
                fatal(f'failed to write output file: {e}')
        else:
            return
        if count < len(data):
            fatal(f'failed to write all output')

    def emit_proto(out_message):
        """Emits the given protobuf message as specified on the command
        line."""

        # Convert to appropriate data format.
        if out_type == 'proto':
            emit_output(out_message.SerializeToString())
        elif out_type == 'json':
            emit_output(json_format.MessageToJson(out_message))
        else:
            out_dict = json_format.MessageToDict(out_message)
            if out_type == 'yaml':
                emit_output(yaml.safe_dump(out_dict))
            elif out_type == 'jsom':
                emit_output(_jsom_dumps(out_dict))
            else:
                fatal(f'cannot emit protobuf message in {out_type} format')

    # Print diagnostic code help if requested.
    if help_diagnostics:
        click.echo('The following diagnostic codes are defined:\n')
        diags = {}
        for code, (name, desc, parent) in sorted(get_diagnostic_codes().items()):
            diag = (code, name, desc, [])
            diags[code] = diag
            if parent is not None:
                diags[parent][3].append(diag)
        def print_diag(diag, first_prefix='', next_prefix=''):
            code, name, desc, children = diag
            click.echo(f'{first_prefix}{code:04d} ({name}): {desc}.')
            for child in children[:-1]:
                print_diag(child, f'{next_prefix} |- ', f'{next_prefix} |  ')
            if children:
                print_diag(children[-1], f'{next_prefix} \'- ', f'{next_prefix}    ')
        print_diag(diags[0])
        sys.exit(0)

    # Parse verbosity level.
    verbosity_level = level_str_to_int(verbosity)

    # Check input file.
    in_file = infile
    if in_file is None:
        click.echo('Missing input file. Try --help for usage information.', err=True)
        sys.exit(2)

    # Handle automatic format deduction.
    in_type = deduce_format(in_file, in_type, {
        'DEFAULT': 'proto',
        'json': 'json',
        'yaml': 'yaml',
        'jsom': 'jsom',
    })
    out_type = deduce_format(out_file, out_type, {
        'DEFAULT': 'proto',
        'json': 'json',
        'yaml': 'yaml',
        'jsom': 'jsom',
        'txt': 'diag',
        'html': 'html',
        'htm': 'html',
    })

    # Read input file.
    if in_file == '-':
        try:
            in_data = sys.stdin.buffer.read()
        except IOError as e:
            fatal(f'failed to read from stdin: {e}')
    else:
        try:
            with open(in_file, 'rb') as f:
                in_data = f.read()
        except IOError as e:
            fatal(f'failed to read input file: {e}')

    # Parse input format.
    if in_type == 'proto':

        # Convert the plan directly.
        try:
            in_plan = load_plan_from_proto(in_data)
        except ProtoDecodeError as e:
            fatal(e)

    else:

        # Remaining formats are UTF-8 encoded.
        try:
            in_str = in_data.decode('utf8')
        except UnicodeError as e:
            fatal(f'failed to decode input file: {e}')

        # Convert from different variations of the JSON object model.
        if in_type == 'json':
            try:
                in_dict = json.loads(in_str)
            except json.decoder.JSONDecodeError as e:
                fatal(f'failed to decode input file: {e}')
        elif in_type == 'yaml':
            try:
                in_dict = yaml.safe_load(in_str)
            except yaml.YAMLError as e:
                fatal(f'failed to decode input file: {e}')
        elif in_type == 'jsom':
            try:
                in_dict = _jsom_loads(in_str)
            except jsom.decoder.DecodeException as e:
                fatal(f'failed to decode input file: {e}')
        else:
            raise NotImplementedError(in_type)

        # The outermost structure must be a dict for anything to work at all.
        if not isinstance(in_dict, dict):
            fatal('toplevel structure of decoded JSON-like input is not a object')

        # Convert the dict representation of the JSON object model to the
        # protobuf message wrapper.
        try:
            in_plan = load_plan_from_dict(in_dict)
        except json_format.ParseError as e:
            fatal(e)

    # Handle convert-only mode.
    if mode == 'convert':
        emit_proto(in_plan)
        return 0

    # Construct parser/validator configuration.
    config = Config()
    if ignore_unknown_fields:
        config.ignore_unknown_fields()
    for pattern in allow_proto_any:
        try:
            config.allow_proto_any_url(pattern)
        except ValueError as e:
            fatal(e)
    for code, minimum, maximum in diagnostic_level:
        try:
            code = int(code, 10)
            if code < 0:
                raise ValueError()
            minimum = minimum.lower()
            if minimum == 'warn':
                minimum = 'warning'
            maximum = maximum.lower()
            if maximum == 'warn':
                maximum = 'warning'
            config.override_diagnostic_level(code, minimum, maximum)
        except ValueError as e:
            fatal(e)
    for pattern, resolve_as in override_uri:
        if resolve_as == '-':
            resolve_as = None
        try:
            config.override_uri(pattern, resolve_as)
        except ValueError as e:
            fatal(e)
    if use_urllib:
        config.add_urllib_resolver()

    # Run the parser/validator.
    result = plan_to_result_handle(in_plan, config)

    # Emit diagnostics to stderr.
    for diagnostic in plan_to_diagnostics(result):
        emit_diagnostic(
            msg=diagnostic.msg,
            code=diagnostic.cause,
            source=path_to_string(diagnostic.path),
            level=diagnostic.adjusted_level,
            original_level=diagnostic.original_level)

    # Check validity.
    validity = check_plan(result)
    if mode == 'loose':
        if validity < 0:
            fatal('plan is invalid')
    elif mode == 'strict':
        if validity < 1:
            fatal('failed to prove that plan is valid')
    elif mode != 'ignore':
        raise ValueError('mode')

    # Emit output file.
    if out_type == 'diag':
        emit_output(plan_to_diagnostics_str(result))
    elif out_type == 'html':
        emit_output(plan_to_html(result))
    else:
        emit_proto(plan_to_parse_result(result))

    return 0


if __name__ == '__main__':
    cli()
