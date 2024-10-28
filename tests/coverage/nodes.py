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
    value: str | int | float | list
    type: str

    def get_base_type(self):
        type = self.type
        if "<" in type:
            type = type[: type.find("<")]
        if type.endswith("?"):
            return type[:-1]
        return type


@dataclass
class TestCase:
    func_name: str
    base_uri: str
    group: CaseGroup
    options: dict
    args: List[CaseLiteral]
    result: CaseLiteral | str
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
        return f"{self.func_name}({', '.join([arg.type for arg in self.args])})"


@dataclass
class TestFile:
    path: str
    version: str
    include: str
    testcases: List[TestCase]
