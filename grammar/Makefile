ANTLR_JAR=antlr-4.13.2-complete.jar
GRAMMARS=SubstraitLexer.g4 FuncTestCaseLexer.g4 FuncTestCaseParser.g4
OUTPUT_DIR=../tests/coverage/antlr_parser

generate:
	java -jar $(ANTLR_JAR) -visitor -Dlanguage=Python3 -o $(OUTPUT_DIR) $(GRAMMARS)

clean:
	rm -rf $(OUTPUT_DIR)/*.py
