package io.substrait.isthmus;

import com.google.common.base.Charsets;
import com.google.common.io.Resources;
import org.apache.calcite.rel.type.RelDataTypeFactory;
import org.apache.calcite.rex.RexBuilder;
import org.apache.calcite.tools.RelBuilder;

import java.io.IOException;

public class PlanTestBase {
  protected final RelCreator creator = new RelCreator();
  protected final RelBuilder builder = creator.createRelBuilder();
  protected final RexBuilder rex = creator.rex();
  protected final RelDataTypeFactory type = creator.type();

  public static String asString(String resource) throws IOException {
    return Resources.toString(
        Resources.getResource(resource), Charsets.UTF_8);
  }
}
