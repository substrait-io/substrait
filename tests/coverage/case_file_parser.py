# SPDX-License-Identifier: Apache-2.0
import os

from antlr4 import CommonTokenStream, FileStream
from antlr4.error.ErrorListener import ErrorListener

from tests.coverage.antlr_parser.FuncTestCaseLexer import FuncTestCaseLexer
from tests.coverage.antlr_parser.FuncTestCaseParser import FuncTestCaseParser
from tests.coverage.visitor import TestCaseVisitor, ParseError


class ParseErrorListener(ErrorListener):
    def __init__(self):
        super(ParseErrorListener, self).__init__()
        self.errors = []

    def syntaxError(self, recognizer, offending_symbol, line, column, msg, e):
        error_message = f"Syntax error at line {line}, column {column}: {msg}"
        self.errors.append(error_message)


def parse_stream(input_stream, file_path):
    # Create a lexer and parser
    lexer = FuncTestCaseLexer(input_stream)
    token_stream = CommonTokenStream(lexer)
    parser = FuncTestCaseParser(token_stream)

    # Add custom error listener
    error_listener = ParseErrorListener()
    parser.removeErrorListeners()
    parser.addErrorListener(error_listener)

    tree = parser.doc()  # This is the entry point of testfile parser
    if parser.getNumberOfSyntaxErrors() > 0:
        print(tree.toStringTree(recog=parser))
        print(f"{parser.getNumberOfSyntaxErrors()} Syntax errors found, exiting")
        raise ParseError(f"Syntax errors: {error_listener.errors}")

    # uncomment below line to see the parse tree for debugging
    # print(tree.toStringTree(recog=parser))

    visitor = TestCaseVisitor(file_path)
    test_file = visitor.visit(tree)
    return test_file


def parse_one_file(file_path):
    return parse_stream(FileStream(file_path, "UTF-8"), file_path)


def parse_testcase_directory_recursively(dir_path):
    # for each file in directory call parse_one_file
    test_files = []
    for child in os.listdir(dir_path):
        child_path = os.path.join(dir_path, child)
        if os.path.isfile(child_path) and child.endswith(".test"):
            test_file = parse_one_file(child_path)
            test_files.append(test_file)
        elif os.path.isdir(child_path):
            test_files_in_a_dir = parse_testcase_directory_recursively(child_path)
            test_files.extend(test_files_in_a_dir)
    return test_files


def load_all_testcases(dir_path) -> list:
    return parse_testcase_directory_recursively(dir_path)
