# SPDX-License-Identifier: Apache-2.0
from dataclasses import dataclass
from typing import List


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

    def get_base_type(self):
        type_str = self.type
        if "<" in type_str:
            type_str = type_str[: type_str.find("<")]
        if type_str.endswith("?"):
            return type_str[:-1]
        return type_str


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
        return [arg.get_base_type() for arg in self.args]

    def get_signature(self):
        return f"{self.func_name}({', '.join([arg.type for arg in self.args])}) = {self.get_return_type()}"


@dataclass
class TestFile:
    path: str
    version: str
    include: str
    testcases: List[TestCase]
