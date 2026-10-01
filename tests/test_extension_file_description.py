# SPDX-License-Identifier: Apache-2.0

import io
import runpy
import shutil
from contextlib import contextmanager
from pathlib import Path

import mkdocs_gen_files
import pytest
import yaml
from jsonschema import Draft202012Validator

REPO = Path(__file__).resolve().parents[1]
GENERATOR = Path("site/docs/extensions/generate_function_docs.py")
EXAMPLE = Path("site/examples/extensions/described_functions.yaml")


@pytest.mark.parametrize("include_description", [True, False])
def test_extension_file_description(tmp_path, monkeypatch, include_description):
    extension = yaml.safe_load((REPO / EXAMPLE).read_text())
    description = extension.pop("description")
    if include_description:
        extension["description"] = description

    schema = yaml.safe_load((REPO / "text/simple_extensions_schema.yaml").read_text())
    Draft202012Validator(schema).validate(extension)

    generator = tmp_path / GENERATOR
    generator.parent.mkdir(parents=True)
    shutil.copyfile(REPO / GENERATOR, generator)
    extensions = tmp_path / "extensions"
    extensions.mkdir()
    (extensions / "example.yaml").write_text(yaml.safe_dump(extension))

    generated = {}

    @contextmanager
    def capture_output(path, _mode):
        buffer = io.StringIO()
        yield buffer
        generated[path] = buffer.getvalue()

    monkeypatch.setitem(mkdocs_gen_files.__dict__, "open", capture_output)
    runpy.run_path(str(generator))

    markdown = generated["extensions/example.md"]
    assert "Negate an integer value." in markdown
    assert "- negate(`i32`): -> `i32`" in markdown
    if include_description:
        assert markdown.count(description.strip()) == 1
        assert markdown.index(description.strip()) < markdown.index(
            "## Scalar Functions"
        )
    else:
        assert description.strip() not in markdown
