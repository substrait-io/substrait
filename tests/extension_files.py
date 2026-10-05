# SPDX-License-Identifier: Apache-2.0
"""Discovery and traversal of the checked-in simple extension YAML files."""

from pathlib import Path

import yaml

REPO_ROOT = Path(__file__).parents[1]
EXTENSIONS_DIR = REPO_ROOT / "extensions"
SITE_EXAMPLES_DIR = REPO_ROOT / "site" / "examples"

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
