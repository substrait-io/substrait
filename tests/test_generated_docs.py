# SPDX-License-Identifier: Apache-2.0
from pathlib import Path

from mkdocs.commands.build import build
from mkdocs.config import load_config


def test_unsigned_integer_function_docs_are_generated(tmp_path, monkeypatch):
    monkeypatch.chdir(Path(__file__).parents[1] / "site")
    config = load_config(config_file="mkdocs.yml", site_dir=str(tmp_path))

    build(config)

    assert (tmp_path / "extensions/unsigned_integers/index.html").is_file()
