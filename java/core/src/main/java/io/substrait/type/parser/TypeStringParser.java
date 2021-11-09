package io.substrait.type.parser;

import io.substrait.function.ParameterizedType;
import io.substrait.type.SubstraitTypeLexer;
import io.substrait.type.SubstraitTypeParser;
import io.substrait.function.TypeExpression;
import io.substrait.type.Type;
import org.antlr.v4.runtime.BaseErrorListener;
import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;
import org.antlr.v4.runtime.RecognitionException;
import org.antlr.v4.runtime.Recognizer;

import java.util.function.Function;

public class TypeStringParser {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(TypeStringParser.class);

  private TypeStringParser() {}

  public static Type parseSimple(String str) {
    return parse(str, ParseToPojo::type);
  }

  public static ParameterizedType parseParameterized(String str) {
    return parse(str, ParseToPojo::parameterizedType);
  }

  public static TypeExpression parseExpression(String str) {
    return parse(str, ParseToPojo::typeExpression);
  }

  private static SubstraitTypeParser.StartContext parse(String str) {
    var lexer = new SubstraitTypeLexer(CharStreams.fromString(str));
    lexer.removeErrorListeners();
    lexer.addErrorListener(TypeErrorListener.INSTANCE);
    var tokenStream = new CommonTokenStream(lexer);
    var parser = new io.substrait.type.SubstraitTypeParser(tokenStream);
    parser.removeErrorListeners();
    parser.addErrorListener(TypeErrorListener.INSTANCE);
    return parser.start();
  }

  public static <T> T parse(String str, Function<SubstraitTypeParser.StartContext, T> func) {
    return func.apply(parse(str));
  }

  public static TypeExpression parse(String str, ParseToPojo.Visitor visitor) {
    return parse(str).accept(visitor);
  }

  private static class TypeErrorListener extends BaseErrorListener {

    public static final TypeErrorListener INSTANCE = new TypeErrorListener();

    @Override public void syntaxError(final Recognizer<?, ?> recognizer, final Object offendingSymbol,
        final int line,
        final int charPositionInLine, final String msg, final RecognitionException e) {
      throw new ParseError(msg, e, line, charPositionInLine);
    }

  }

  public static class ParseError extends RuntimeException {
    private final int line;
    private final int posInLine;

    public ParseError(final String message, final Throwable cause, final int line, final int posInLine) {
      super(message, cause);
      this.line = line;
      this.posInLine = posInLine;
    }
  }
}
