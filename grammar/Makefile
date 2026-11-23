TYPE_GRAMMAR=SubstraitLexer.g4 SubstraitType.g4
TYPE_OUTPUT_DIR=../tests/type/antlr_parser
GO_OUTPUT_DIR=../gen/antlr/go
JAVA_OUTPUT_DIR=../gen/antlr/java
CPP_OUTPUT_DIR=../gen/antlr/cpp
CSHARP_OUTPUT_DIR=../gen/antlr/csharp
TESTCASE_GRAMMAR=FuncTestCaseLexer.g4 FuncTestCaseParser.g4
TESTCASE_OUTPUT_DIR=../tests/parser/antlr_parser

.PHONY: all generate_python_parser generate_testcase_parser generate_type_parser generate_go_parser generate_java_parser generate_cpp_parser generate_csharp_parser clean

all: generate_python_parser generate_go_parser generate_java_parser generate_cpp_parser generate_csharp_parser

generate_python_parser: generate_testcase_parser generate_type_parser

generate_testcase_parser:
	@echo "\nGenerating Test Case Parser"
	antlr4  -visitor -Dlanguage=Python3 -o $(TESTCASE_OUTPUT_DIR) $(TESTCASE_GRAMMAR)
	rm -rf $(TESTCASE_OUTPUT_DIR)/*.tokens $(TESTCASE_OUTPUT_DIR)/*.interp
	./prepend_license.sh $(TESTCASE_OUTPUT_DIR)

generate_type_parser:
	@echo "\nGenerating Substrait Type"
	antlr4 -visitor -Dlanguage=Python3 -o $(TYPE_OUTPUT_DIR) $(TYPE_GRAMMAR)
	rm -rf $(TYPE_OUTPUT_DIR)/*.tokens $(TYPE_OUTPUT_DIR)/*.interp
	./prepend_license.sh $(TYPE_OUTPUT_DIR)

generate_go_parser:
	@echo "\nGenerating Go parsers"
	mkdir -p $(GO_OUTPUT_DIR)/substraittype $(GO_OUTPUT_DIR)/functestcase
	antlr4 -o $(GO_OUTPUT_DIR)/substraittype -Dlanguage=Go -visitor -package substraittype $(TYPE_GRAMMAR)
	antlr4 -o $(GO_OUTPUT_DIR)/functestcase -Dlanguage=Go -visitor -no-listener -package functestcase $(TESTCASE_GRAMMAR)
	cp ../tests/antlr/go.mod $(GO_OUTPUT_DIR)/go.mod

generate_java_parser:
	@echo "\nGenerating Java parsers"
	antlr4 -o $(JAVA_OUTPUT_DIR) -Dlanguage=Java -visitor $(TYPE_GRAMMAR) $(TESTCASE_GRAMMAR)

generate_cpp_parser:
	@echo "\nGenerating C++ parsers"
	antlr4 -o $(CPP_OUTPUT_DIR) -Dlanguage=Cpp -visitor $(TYPE_GRAMMAR) $(TESTCASE_GRAMMAR)

generate_csharp_parser:
	@echo "\nGenerating C# parsers"
	antlr4 -o $(CSHARP_OUTPUT_DIR) -Dlanguage=CSharp -visitor $(TYPE_GRAMMAR) $(TESTCASE_GRAMMAR)
	cp ../tests/antlr/AntlrCheck.csproj $(CSHARP_OUTPUT_DIR)/AntlrCheck.csproj

clean:
	rm -rf $(TYPE_OUTPUT_DIR)/*.py  $(TYPE_OUTPUT_DIR)/*.tokens $(TYPE_OUTPUT_DIR)/*.interp
	rm -rf $(TESTCASE_OUTPUT_DIR)/*.py $(TESTCASE_OUTPUT_DIR)/*.tokens $(TESTCASE_OUTPUT_DIR)/*.interp
	rm -rf ../gen/antlr
	rm -rf ./*.tokens
