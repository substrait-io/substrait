# SPDX-License-Identifier: Apache-2.0
from dataclasses import dataclass
from typing import List


def type_str_is_outer_nullable(type_str):
    """Whether a type string has a nullable outer type.

    The ``?`` marker appears at the end for scalar types (e.g. ``i32?``)
    and before ``<`` for parameterized types (e.g. ``list?<i32>``).
    Inner element nullability (e.g. ``list<i32?>``) is not outer-nullable.

    TODO: In the visitor, this should ideally come from the parse tree's
    ``isnull`` token rather than re-parsing the string, but ``dataType``
    is an intermediate rule and the ``isnull`` token lives on concrete
    type contexts nested several levels deep.  Getting it would require a
    non-trivial refactor of the visitor to propagate nullability up
    through ``dataType``.
    """
    bracket_pos = type_str.find("<")
    if bracket_pos == -1:
        return type_str.endswith("?")
    return "?" in type_str[:bracket_pos]


@dataclass
class CaseGroup:
    name: str
    description: str


@dataclass
class SubstraitError:
    error: str


@dataclass
class CaseLiteral:
    value: str | int | float | list | None
    type: str
    nullable: bool = False

    def get_base_type(self):
        type_str = self.type
        if "<" in type_str:
            type_str = type_str[: type_str.find("<")]
        type_str = type_str.rstrip("?")
        return type_str

    def is_nullable(self):
        """Whether the outer type is nullable.

        Set during parsing from the grammar's isnull token, which only
        appears on the outermost type.  For example list?<i32> is
        nullable but list<i32?> is not.
        """
        return self.nullable


@dataclass
class AggregateArgument:
    column_name: str
    column_type: str
    table_name: str
    scalar_value: CaseLiteral | None


@dataclass
class TestCase:
    func_name: str
    base_uri: str
    group: CaseGroup | None
    options: dict
    rows: List[List] | None
    args: List[CaseLiteral] | List[AggregateArgument]
    result: CaseLiteral | str | SubstraitError
    comment: str

    def get_return_type(self):
        if isinstance(self.result, CaseLiteral):
            return self.result.type
        return self.result

    def is_return_type_error(self):
        return isinstance(self.result, SubstraitError)

    def get_arg_types(self):
        types = []
        for arg in self.args:
            if isinstance(arg, CaseLiteral):
                types.append(arg.get_base_type())
            elif isinstance(arg, AggregateArgument):
                # For aggregate arguments, use column_type if available, otherwise extract from scalar_value
                if arg.column_type:
                    types.append(arg.column_type)
                elif arg.scalar_value:
                    types.append(arg.scalar_value.get_base_type())
        return types

    def get_signature(self):
        arg_types = []
        for arg in self.args:
            if isinstance(arg, CaseLiteral):
                arg_types.append(arg.type)
            elif isinstance(arg, AggregateArgument):
                if arg.column_type:
                    arg_types.append(arg.column_type)
                elif arg.scalar_value:
                    arg_types.append(arg.scalar_value.type)
        return f"{self.func_name}({', '.join(arg_types)}) = {self.get_return_type()}"


@dataclass
class TestFile:
    path: str
    version: str
    include: str  # Primary extension being tested
    dependencies: List[str]  # Additional extensions needed for tests
    testcases: List[TestCase]
