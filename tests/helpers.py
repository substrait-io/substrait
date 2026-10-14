# SPDX-License-Identifier: Apache-2.0
from pathlib import Path

import yaml
from antlr4 import InputStream
from tests.parser import parse_stream

TESTS_DIR = Path(__file__).parent
REPO_ROOT = TESTS_DIR.parent
EXTENSIONS_DIR = REPO_ROOT / "extensions"
SITE_EXAMPLES_DIR = REPO_ROOT / "site" / "examples"


def parse_string(input_string):
    return parse_stream(InputStream(input_string), "test_string")


def make_scalar_header(version, include):
    return f"""### SUBSTRAIT_SCALAR_TEST: {version}
### SUBSTRAIT_INCLUDE: {include}

"""


def make_aggregate_test_header(version, include):
    return f"""### SUBSTRAIT_AGGREGATE_TEST: {version}
### SUBSTRAIT_INCLUDE: {include}

"""


def make_window_test_header(version, include):
    return f"""### SUBSTRAIT_WINDOW_TEST: {version}
### SUBSTRAIT_INCLUDE: {include}

"""


def get_test_path(relative_path):
    return str(TESTS_DIR / relative_path)


FUNCTION_SECTIONS = ("scalar_functions", "aggregate_functions", "window_functions")


def find_extension_files(*roots):
    """Paths of the extension YAML files under each root, sorted per root."""
    files = []
    for root in roots:
        files.extend(sorted(Path(root).rglob("*.yaml")))
    return files


def load_extension(path):
    """Parse one extension YAML file into a dict."""
    with open(path) as fh:
        return yaml.load(fh, Loader=yaml.FullLoader)


def iter_function_impls(extension):
    """Yield ``(function, impl)`` for every function impl in an extension document."""
    for section in FUNCTION_SECTIONS:
        for function in extension.get(section) or []:
            for impl in function.get("impls") or []:
                yield function, impl
