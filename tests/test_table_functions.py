# SPDX-License-Identifier: Apache-2.0

import pytest
import yaml
from tests.coverage.extensions import validate_nullability_markers


@pytest.mark.parametrize(
    "argument_type, expected_error_count",
    [("list?<i64>", 2), ("list<i64?>", 0)],
)
def test_table_function_declarations_use_declared_output_validation(
    tmp_path, argument_type, expected_error_count
):
    extension = {
        "urn": "extension:org.example:table_functions",
        "table_functions": [
            {
                "name": "generate",
                "impls": [
                    {
                        "args": [{"name": "input", "value": argument_type}],
                        "schema": "struct<i64?>",
                    },
                    {"args": [{"name": "input", "value": argument_type}]},
                ],
            }
        ],
    }
    (tmp_path / "functions_table.yaml").write_text(yaml.safe_dump(extension))
    errors = validate_nullability_markers(tmp_path)
    assert len(errors) == expected_error_count
    assert all("DECLARED_OUTPUT" in error for error in errors)
