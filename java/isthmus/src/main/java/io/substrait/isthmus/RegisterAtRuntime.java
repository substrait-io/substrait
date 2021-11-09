package io.substrait.isthmus;

import com.google.protobuf.GeneratedMessageV3;
import com.google.protobuf.MessageLite;
import com.google.protobuf.ProtocolMessageEnum;
import io.substrait.function.SimpleExtension;
import org.apache.calcite.rel.metadata.*;
import org.apache.calcite.runtime.CalciteContextException;
import org.apache.calcite.runtime.Resources;
import org.apache.calcite.sql.fun.SqlStdOperatorTable;
import org.apache.calcite.sql.validate.SqlValidatorException;
import org.apache.calcite.sql2rel.StandardConvertletTable;
import org.apache.calcite.util.BuiltInMethod;
import org.graalvm.nativeimage.hosted.Feature;
import org.graalvm.nativeimage.hosted.RuntimeReflection;
import org.immutables.value.Value;
import org.reflections.Reflections;
import org.reflections.scanners.FieldAnnotationsScanner;
import org.reflections.scanners.SubTypesScanner;
import org.reflections.scanners.TypeAnnotationsScanner;

import java.lang.annotation.Annotation;
import java.util.Arrays;

public class RegisterAtRuntime implements Feature {
  public void beforeAnalysis(BeforeAnalysisAccess access) {
    try {
      Reflections substrait = new Reflections("io.substrait");
      // cli picocli
      register(PlanEntryPoint.class);

      //protobuf items
      registerByParent(substrait, GeneratedMessageV3.class);
      registerByParent(substrait, MessageLite.Builder.class);
      registerByParent(substrait, ProtocolMessageEnum.class);

      // Substrait immutables.
      registerByAnnotation(substrait, Value.Immutable.class);

      // Records
      register(SimpleExtension.TypeArgument.class);
      register(SimpleExtension.EnumArgument.class);
      register(SimpleExtension.ValueArgument.class);

      // calcite items
      Reflections calcite = new Reflections("org.apache.calcite", new FieldAnnotationsScanner(), new SubTypesScanner());
      register(BuiltInMetadata.class);
      register(SqlValidatorException.class);
      register(CalciteContextException.class);
      register(SqlStdOperatorTable.class);
      register(StandardConvertletTable.class);
      registerByParent(calcite, Metadata.class);
      registerByParent(calcite, MetadataHandler.class);
      registerByParent(calcite, Resources.Element.class);

      Arrays.asList(RelMdPercentageOriginalRows.class,
          RelMdColumnOrigins.class,
          RelMdExpressionLineage.class,
          RelMdTableReferences.class,
          RelMdNodeTypes.class,
          RelMdRowCount.class,
          RelMdMaxRowCount.class,
          RelMdMinRowCount.class,
          RelMdUniqueKeys.class,
          RelMdColumnUniqueness.class,
          RelMdPopulationSize.class,
          RelMdSize.class,
          RelMdParallelism.class,
          RelMdDistribution.class,
          RelMdLowerBoundCost.class,
          RelMdMemory.class,
          RelMdDistinctRowCount.class,
          RelMdSelectivity.class,
          RelMdExplainVisibility.class,
          RelMdPredicates.class,
          RelMdAllPredicates.class,
          RelMdCollation.class)
          .forEach(RegisterAtRuntime::register);

      RuntimeReflection.register(Resources.class);
      RuntimeReflection.register(SqlValidatorException.class);

      Arrays.stream(BuiltInMethod.values()).forEach(c -> {
        if (c.field != null) RuntimeReflection.register(c.field);
        if (c.constructor != null) RuntimeReflection.register(c.constructor);
        if (c.method != null) RuntimeReflection.register(c.method);
      });
    } catch (Exception e) {
      throw new RuntimeException(e);
    }
  }

  private static void register(Class<?> c) {
    RuntimeReflection.register(c);
    RuntimeReflection.register(c.getDeclaredConstructors());
    RuntimeReflection.register(c.getDeclaredFields());
    RuntimeReflection.register(c.getDeclaredMethods());
    RuntimeReflection.register(c.getConstructors());
    RuntimeReflection.register(c.getFields());
    RuntimeReflection.register(c.getMethods());
  }

  private static void registerByAnnotation(Reflections reflections, Class<? extends Annotation> c) {
    reflections.getTypesAnnotatedWith(c).stream().forEach(inner -> {
      register(inner);
      reflections.getSubTypesOf(c).stream().forEach(RegisterAtRuntime::register);
    });
  }

  private static void registerByParent(Reflections reflections, Class<?> c) {
    register(c);
    reflections.getSubTypesOf(c).stream().forEach(RegisterAtRuntime::register);
  }


  ;
}
