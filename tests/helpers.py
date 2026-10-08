# SPDX-License-Identifier: Apache-2.0
import os

from antlr4 import InputStream
from tests.parser import parse_stream


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
    tests_dir = os.path.dirname(os.path.abspath(__file__))
    return os.path.join(tests_dir, relative_path)
