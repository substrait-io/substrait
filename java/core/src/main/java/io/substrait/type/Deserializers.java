package io.substrait.type;

import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonMappingException;
import com.fasterxml.jackson.databind.deser.std.StdDeserializer;
import com.fasterxml.jackson.databind.module.SimpleModule;

import io.substrait.function.ParameterizedType;
import io.substrait.function.TypeExpression;
import io.substrait.type.parser.ParseToPojo;
import io.substrait.type.parser.TypeStringParser;

import java.io.IOException;
import java.util.function.Function;

public class Deserializers {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(Deserializers.class);

  public static final StdDeserializer<ParameterizedType> PARAMETERIZED_TYPE =
      new ParseDeserializer<>(ParameterizedType.class, ParseToPojo::parameterizedType);
  public static final StdDeserializer<Type> TYPE =
      new ParseDeserializer<>(Type.class, ParseToPojo::type);
  public static final StdDeserializer<TypeExpression> DERIVATION_EXPRESSION =
      new ParseDeserializer<>(TypeExpression.class, ParseToPojo::typeExpression);

  public static final SimpleModule MODULE = new SimpleModule()
      .addDeserializer(ParameterizedType.class, PARAMETERIZED_TYPE)
      .addDeserializer(TypeExpression.class, DERIVATION_EXPRESSION);

  public static class ParseDeserializer<T> extends StdDeserializer<T> {

    private final Function<SubstraitTypeParser.StartContext, T> converter;
    public ParseDeserializer(Class<T> clazz, Function<SubstraitTypeParser.StartContext, T> converter) {
      super(clazz);
      this.converter = converter;
    }

    @Override
    public T deserialize(final JsonParser p, final DeserializationContext ctxt)
        throws IOException, JsonProcessingException {
      var typeString = p.getValueAsString();
      try {
        return TypeStringParser.parse(typeString, converter);
      } catch (Exception ex) {
        throw JsonMappingException.from(p, "Unable to parse string " + typeString.replace("\n", " \\n"), ex);
      }
    }
  }

}
