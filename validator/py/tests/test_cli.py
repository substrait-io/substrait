# SPDX-License-Identifier: Apache-2.0

from click.testing import CliRunner
from substrait_validator import cli
from data import BASIC_PLAN
import tempfile
import json
import pprint
from os.path import join as pjoin
from os.path import isfile
import platform


def run(*args):
    return CliRunner().invoke(cli, args)


def test_no_args():
    result = run()
    assert result.exit_code == 2
    assert 'Missing input file.' in result.output


def test_mconvert_auto():
    """Test -mconvert with automatic format deduction from file extensions."""
    with tempfile.TemporaryDirectory() as tmp:
        with open(pjoin(tmp, 'plan.json'), 'w') as f:
            f.write(BASIC_PLAN)

        def convert(src, dest):
            assert run(pjoin(tmp, src), '-O', pjoin(tmp, dest), '-mconvert').exit_code == 0

        convert('plan.json', 'plan.proto')

        with open(pjoin(tmp, 'plan.proto'), 'rb') as f:
            a = f.read()

        convert('plan.proto', 'plan.yaml')
        #convert('plan.yaml', 'plan.jsom')
        convert('plan.yaml', 'plan.json') # FIXME: JSOM decoder isn't round-tripping again
        convert('plan.json', 'plan.bin')

        with open(pjoin(tmp, 'plan.bin'), 'rb') as f:
            b = f.read()

        assert a == b


def test_mconvert_manual():
    """Test -mconvert with automatic format deduction from file extensions."""
    with tempfile.TemporaryDirectory() as tmp:
        with open(pjoin(tmp, 'data'), 'w') as f:
            f.write(BASIC_PLAN)

        def convert(in_type, out_type):
            assert run(pjoin(tmp, 'data'), '-O', pjoin(tmp, 'data'), '-mconvert',
                       '--in-type', in_type, '--out-type', out_type).exit_code == 0

        convert('json', 'proto')

        with open(pjoin(tmp, 'data'), 'rb') as f:
            a = f.read()

        convert('proto', 'yaml')
        #convert('yaml', 'jsom')
        convert('yaml', 'json') # FIXME: JSOM decoder isn't round-tripping again
        convert('json', 'proto')

        with open(pjoin(tmp, 'data'), 'rb') as f:
            b = f.read()

        assert a == b


def test_valid_invalid():
    """Test exit code based on validity for various modes using diagnostic
    level overrides to force an outcome."""
    with tempfile.TemporaryDirectory() as tmp:
        with open(pjoin(tmp, 'plan.json'), 'w') as f:
            f.write(BASIC_PLAN)

        # Test all corner cases.
        def x(mode, level):
            return run(pjoin(tmp, 'plan.json'), '-m', mode, '--diagnostic-level', '0', level, level).exit_code

        assert x('ignore', 'error') == 0
        assert x('loose', 'error') == 1
        assert x('loose', 'warning') == 0
        assert x('strict', 'warning') == 1
        assert x('strict', 'info') == 0

        # Default should be -mloose.
        def x(level):
            return run(pjoin(tmp, 'plan.json'), '--diagnostic-level', '0', level, level).exit_code

        assert x('info') == 0
        assert x('warning') == 0
        assert x('error') == 1


def test_verbosity():
    """Test verbosity using diagnostic level overrides."""
    with tempfile.TemporaryDirectory() as tmp:
        with open(pjoin(tmp, 'plan.json'), 'w') as f:
            f.write(BASIC_PLAN)

        # Test all corner cases.
        def x(verbosity, level):
            return run(pjoin(tmp, 'plan.json'), '-v', verbosity, '--diagnostic-level', '0', level, level).output.split(maxsplit=1)[:1]

        assert x('quiet', 'error') == []
        assert x('fatal', 'error') == ['Fatal']
        assert x('error', 'error') == ['Error']
        assert x('error', 'warn') == []
        assert x('warn', 'warn') == ['Warning']
        assert x('warn', 'info') == []
        assert x('info', 'info') == ['Info']


def test_export():
    """Test export logic."""
    with tempfile.TemporaryDirectory() as tmp:
        with open(pjoin(tmp, 'plan.json'), 'w') as f:
            f.write(BASIC_PLAN)

        def x(output, level):
            return run(pjoin(tmp, 'plan.json'), '-O', pjoin(tmp, output), '--diagnostic-level', '0', level, level).exit_code

        def y(output):
            assert x(output, 'error') == 1
            assert not isfile(pjoin(tmp, output))
            assert x(output, 'info') == 0
            with open(pjoin(tmp, output), 'rb') as f:
                return f.read()

        assert y('output.proto')[0] == 10
        assert y('output.json').startswith(b'{\n  "root":')
        assert y('output.yaml').startswith(b'root:')
        assert y('output.jsom').startswith(b'@macros')
        assert b'<!DOCTYPE html>' in y('output.html')
        assert y('output.txt').startswith(b'Info')


def test_uri_resolution():
    """Test URI resolution logic."""
    with tempfile.TemporaryDirectory() as tmp:
        with open(pjoin(tmp, 'plan.json'), 'w') as f:
            f.write(json.dumps({
                'extensionUris': [
                    {
                        'extension_uri_anchor': 1,
                        'uri': 'https://raw.githubusercontent.com/substrait-io/substrait/82078995c19faa9d4e53a90cd66800c26d88f970/extensions/extension_types.yaml'
                    }
                ]
            }))

        # Obtain a valid file:// URL for the above JSON file as well.
        if platform.system() == 'Windows':
            local_url = 'file:///' + pjoin(tmp, 'plan.json').replace('\\', '/')
        else:
            local_url = 'file://' + pjoin(tmp, 'plan.json')

        def x(*args):
            return run(pjoin(tmp, 'plan.json'), '-verror',              # verbosity error
                       '--diagnostic-level', '2002', 'error', 'error',  # YAML resolution failure -> error
                       '--diagnostic-level', '0', 'info', 'info',       # all other diagnostics -> info
                       *args).exit_code

        # Actual remote lookup.
        assert x() == 0

        # Disable remote lookups, so we expect a failure (not file://).
        assert x('--no-use-urllib') == 1

        # Try file:// protocol instead. This one is handled by the Rust
        # fallback resolution logic. Note that plan.json is obviously not
        # valid YAML, but all diagnostics not related to URI resolution are
        # overridden to info, so we don't have to care.
        assert x('--no-use-urllib', '--override-uri', '*', local_url) == 0

        # urllib should also support file://.
        assert x('--use-urllib', '--override-uri', '*', local_url) == 0
