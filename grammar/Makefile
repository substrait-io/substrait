ANTLR_JAR=antlr-4.13.2-complete.jar
TYPE_GRAMMAR=SubstraitLexer.g4 SubstraitType.g4
TYPE_OUTPUT_DIR=../tests/type/antlr_parser
TESTCASE_GRAMMAR=FuncTestCaseLexer.g4 FuncTestCaseParser.g4
TESTCASE_OUTPUT_DIR=../tests/coverage/antlr_parser

generate_testcase_parser:
	java -jar $(ANTLR_JAR) -visitor -Dlanguage=Python3 -o $(TESTCASE_OUTPUT_DIR) $(TESTCASE_GRAMMAR)

generate_type_parser:
	java -jar $(ANTLR_JAR) -visitor -Dlanguage=Python3 -o $(TYPE_OUTPUT_DIR) $(TYPE_GRAMMAR)

clean:
	rm -rf $(TYPE_OUTPUT_DIR)/*.py  $(TYPE_OUTPUT_DIR)/*.tokens $(TYPE_OUTPUT_DIR)/*.interp
	rm -rf $(TESTCASE_OUTPUT_DIR)/*.py $(TESTCASE_OUTPUT_DIR)/*.tokens $(TESTCASE_OUTPUT_DIR)/*.interp
