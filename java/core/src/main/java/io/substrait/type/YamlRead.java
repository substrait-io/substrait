package io.substrait.type;

import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;

import io.substrait.function.SimpleExtension;

import java.io.File;
import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class YamlRead {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(YamlRead.class);

  private final static List<String> FUNCTIONS = Collections.unmodifiableList(Arrays.asList(
     "boolean", "aggregate_generic", "arithmetic_decimal", "arithmetic", "comparison", "datetime", "string"
  ));
  public static void main(String[] args) throws Exception {
    try {
      System.out.println("Read: " + YamlRead.class.getResource("/functions_boolean.yaml"));
      List<SimpleExtension.Function> signatures = loadFunctions();

      signatures.forEach(f -> System.out.println(f.key()));
    } catch (Exception ex){
      throw ex;
    }
  }

  public static List<SimpleExtension.Function> loadFunctions() {
    return loadFunctions(FUNCTIONS.stream().map(c -> String.format("/src/substrait/extensions/functions_%s.yaml", c)).toList());
  }

  public static List<SimpleExtension.Function> loadFunctions(List<String> files) {
     return files.stream().flatMap(YamlRead::parse)
         .collect(Collectors.toList());
  }

  private static Stream<SimpleExtension.Function> parse(String name) {

    try {
      ObjectMapper mapper = new ObjectMapper(new YAMLFactory())
          .enable(DeserializationFeature.ACCEPT_SINGLE_VALUE_AS_ARRAY)
          .registerModule(Deserializers.MODULE);
      var doc = mapper.readValue(new File(name), SimpleExtension.FunctionSignatures.class);

      logger.debug("Parsed {} functions in file {}.",
          Optional.ofNullable(doc.scalars()).map(List::size).orElse(0) +
              Optional.ofNullable(doc.aggregates()).map(List::size).orElse(0), name);

      return doc.resolve(name);
    } catch (RuntimeException ex) {
      throw ex;
    } catch (Exception ex) {
      throw new RuntimeException("Failure while parsing file " + name, ex);
    }
  }

}
