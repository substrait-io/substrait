package io.substrait.isthmus;

import org.apache.calcite.jdbc.JavaTypeFactoryImpl;
import org.apache.calcite.rel.type.RelDataTypeFactory;
import org.apache.calcite.rel.type.RelDataTypeSystem;
import org.apache.calcite.rel.type.RelDataTypeSystemImpl;
import org.apache.calcite.sql.type.SqlTypeName;

public class SubstraitTypeSystem extends RelDataTypeSystemImpl {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(
      SubstraitTypeSystem.class);

  public static final RelDataTypeSystem TYPE_SYSTEM = new SubstraitTypeSystem();

  private SubstraitTypeSystem() {}

  @Override public int getMaxPrecision(final SqlTypeName typeName) {
    switch (typeName){
      case INTERVAL_DAY:
      case INTERVAL_YEAR:
      case INTERVAL_YEAR_MONTH:
      case TIME:
      case TIME_WITH_LOCAL_TIME_ZONE:
      case TIMESTAMP:
      case TIMESTAMP_WITH_LOCAL_TIME_ZONE:
        return 6;
    }
    return super.getMaxPrecision(typeName);
  }

  @Override public int getMaxNumericScale() {
    return 38;
  }

  @Override public int getMaxNumericPrecision() {
    return 38;
  }

  public static RelDataTypeFactory createTypeFactory() {
    return new JavaTypeFactoryImpl(TYPE_SYSTEM);
  }
}
