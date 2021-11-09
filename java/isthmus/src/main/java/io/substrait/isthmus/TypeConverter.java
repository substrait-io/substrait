package io.substrait.isthmus;

import io.substrait.function.NullableType;
import io.substrait.type.NamedStruct;
import io.substrait.type.Type;
import io.substrait.type.TypeCreator;
import io.substrait.type.TypeVisitor;
import org.apache.calcite.avatica.util.TimeUnit;
import org.apache.calcite.rel.type.RelDataType;
import org.apache.calcite.rel.type.RelDataTypeFactory;
import org.apache.calcite.sql.SqlIntervalQualifier;
import org.apache.calcite.sql.parser.SqlParserPos;
import org.apache.calcite.sql.type.MapSqlType;
import org.apache.calcite.sql.type.SqlTypeName;

import io.substrait.function.TypeExpression;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class TypeConverter {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(TypeConverter.class);

  static final SqlIntervalQualifier INTERVAL_YEAR = new SqlIntervalQualifier(TimeUnit.YEAR, TimeUnit.MONTH, SqlParserPos.ZERO);
  static final SqlIntervalQualifier INTERVAL_DAY = new SqlIntervalQualifier(TimeUnit.DAY, TimeUnit.SECOND, SqlParserPos.ZERO);

  public static Type convert(RelDataType type) {
    return convert(type, new ArrayList<>());
  }

  public static NamedStruct toNamedStruct(RelDataType type) {
    if(type.getSqlTypeName() != SqlTypeName.ROW) {
      throw new IllegalArgumentException("Expected type of struct.");
    }

    var names = new ArrayList<String>();
    var struct = (Type.Struct) convert(type, names);
    return NamedStruct.of(names, struct);
  }

  private static Type convert(RelDataType type, List<String> names) {
    TypeCreator creator = Type.withNullability(type.isNullable());
    return switch (type.getSqlTypeName()) {
      case BOOLEAN -> creator.BOOLEAN;
      case TINYINT -> creator.I8;
      case SMALLINT -> creator.I16;
      case INTEGER -> creator.I32;
      case BIGINT -> creator.I64;
      case FLOAT -> creator.FP32;
      case DOUBLE -> creator.FP64;
      case DECIMAL -> {
        if (type.getPrecision() > 38) {
          throw new UnsupportedOperationException("unsupported decimal precision " + type.getPrecision());
        }
        yield creator.decimal(type.getPrecision(), type.getScale());
      }
      case CHAR -> creator.fixedChar(type.getPrecision());
      case VARCHAR -> {
        if (type.getPrecision() == RelDataType.PRECISION_NOT_SPECIFIED) {
          yield creator.STRING;
        }
        yield creator.varChar(type.getPrecision());
      }
      case SYMBOL -> creator.STRING;
      case DATE -> creator.DATE;
      case TIME -> {
        if (type.getPrecision() != 6) {
          throw new UnsupportedOperationException("unsupported time precision " + type.getPrecision());
        }
        yield creator.TIME;
      }
      case TIMESTAMP -> {
        if (type.getPrecision() != 6) {
          throw new UnsupportedOperationException("unsupported timestamp precision " + type.getPrecision());
        }
        yield creator.TIMESTAMP;
      }
      case TIMESTAMP_WITH_LOCAL_TIME_ZONE -> {
        if (type.getPrecision() != 6) {
          throw new UnsupportedOperationException("unsupported timestamptz precision " + type.getPrecision());
        }
        yield creator.TIMESTAMP_TZ;
      }
      case INTERVAL_YEAR, INTERVAL_YEAR_MONTH, INTERVAL_MONTH -> creator.INTERVAL_YEAR;
      case INTERVAL_DAY, INTERVAL_DAY_HOUR, INTERVAL_DAY_MINUTE, INTERVAL_DAY_SECOND, INTERVAL_HOUR, INTERVAL_HOUR_MINUTE,
          INTERVAL_HOUR_SECOND, INTERVAL_MINUTE, INTERVAL_MINUTE_SECOND, INTERVAL_SECOND -> creator.INTERVAL_DAY;
      case VARBINARY -> creator.BINARY;
      case BINARY -> creator.fixedBinary(type.getPrecision());
      case MAP -> {
        MapSqlType map = (MapSqlType) type;
        yield creator.map(convert(map.getKeyType(), names), convert(map.getValueType(), names));
      }
      case ROW -> {
        var children = new ArrayList<Type>();
        for (var field : type.getFieldList()) {
            names.add(field.getName());
            children.add(convert(field.getType(), names));
        }
        yield creator.struct(children);
      }
      case ARRAY -> creator.list(convert(type.getComponentType(), names));
      default -> throw new UnsupportedOperationException(String.format("Unable to convert the type " + type.toString()));
    };
  }

  public static RelDataType convert(RelDataTypeFactory type, TypeExpression typeExpression) {
    return convert(type, typeExpression, null);
  }

  public static RelDataType convert(RelDataTypeFactory type, TypeExpression typeExpression, List<String> dfsFieldNames) {
      return typeExpression.accept(new ToRelDataType(type, dfsFieldNames, 0));
  }

  private static class ToRelDataType extends TypeVisitor.TypeThrowsVisitor<RelDataType, RuntimeException> {

    private final RelDataTypeFactory typeFactory;
    private final List<String> fieldNames;
    private int fieldNamePosition;
    private boolean withinStruct;

    public ToRelDataType(final RelDataTypeFactory type, final List<String> fieldNames, int fieldNamePosition) {
      super("Unknown expression type.");
      this.typeFactory = type;
      this.fieldNames = fieldNames;
      this.fieldNamePosition = fieldNamePosition;
    }

    @Override public RelDataType visit(Type.Bool expr) {
      return t(n(expr), SqlTypeName.BOOLEAN);
    }

    @Override public RelDataType visit(Type.I8 expr) {
      return t(n(expr), SqlTypeName.TINYINT);
    }

    @Override public RelDataType visit(Type.I16 expr) {
      return t(n(expr), SqlTypeName.SMALLINT);
    }

    @Override public RelDataType visit(Type.I32 expr) {
      return t(n(expr), SqlTypeName.INTEGER);
    }

    @Override public RelDataType visit(Type.I64 expr) {
      return t(n(expr), SqlTypeName.BIGINT);
    }

    @Override public RelDataType visit(Type.FP32 expr) {
      return t(n(expr), SqlTypeName.FLOAT);
    }

    @Override public RelDataType visit(Type.FP64 expr) {
      return t(n(expr), SqlTypeName.DOUBLE);
    }

    @Override public RelDataType visit(Type.Str expr) {
      return t(n(expr), SqlTypeName.VARCHAR);
    }

    @Override public RelDataType visit(Type.Binary expr) {
      return t(n(expr), SqlTypeName.VARBINARY);
    }

    @Override public RelDataType visit(Type.Date expr) {
      return t(n(expr), SqlTypeName.DATE);
    }

    @Override public RelDataType visit(Type.Time expr) {
      return t(n(expr), SqlTypeName.TIME, 6);
    }

    @Override public RelDataType visit(Type.TimestampTZ expr) {
      return t(n(expr), SqlTypeName.TIMESTAMP_WITH_LOCAL_TIME_ZONE, 6);
    }

    @Override public RelDataType visit(Type.Timestamp expr) {
      return t(n(expr), SqlTypeName.TIMESTAMP, 6);
    }

    @Override public RelDataType visit(Type.IntervalYear expr) {
      return typeFactory.createTypeWithNullability(typeFactory.createSqlIntervalType(INTERVAL_YEAR), n(expr));
    }

    @Override public RelDataType visit(Type.IntervalDay expr) {
      return typeFactory.createTypeWithNullability(typeFactory.createSqlIntervalType(INTERVAL_DAY), n(expr));
    }

    @Override public RelDataType visit(Type.FixedChar expr) {
      return t(n(expr), SqlTypeName.CHAR, expr.length());
    }

    @Override public RelDataType visit(Type.VarChar expr) {
      return t(n(expr), SqlTypeName.VARCHAR, expr.length());
    }

    @Override public RelDataType visit(Type.FixedBinary expr) {
      return t(n(expr), SqlTypeName.BINARY, expr.length());
    }

    @Override public RelDataType visit(Type.Decimal expr) {
      return t(n(expr), SqlTypeName.DECIMAL, expr.precision(), expr.scale());
    }

    @Override public RelDataType visit(Type.Struct expr) {
      if (withinStruct) {
        throw new IllegalStateException("Visitor can't be re-used for nested structs.");
      }
      withinStruct = true;
      try {
        List<RelDataType> fieldTypes = new ArrayList<>();
        List<String> localFieldNames = new ArrayList<>();
        for (TypeExpression field : expr.fields()) {
          localFieldNames.add(fieldNames == null ? "f" + fieldNamePosition : fieldNames.get(fieldNamePosition));
          fieldNamePosition++;
          ToRelDataType childVisitor = new ToRelDataType(typeFactory, fieldNames, fieldNamePosition);
          fieldTypes.add(field.accept(childVisitor));
          fieldNamePosition = childVisitor.fieldNamePosition;
        }

        return n(expr, typeFactory.createStructType(fieldTypes, localFieldNames));

      } finally {
        withinStruct = false;
      }
    }

    @Override public RelDataType visit(Type.ListType expr) {
      return n(expr, typeFactory.createArrayType(expr.elementType().accept(this), -1));
    }

    @Override public RelDataType visit(Type.Map expr) {
      return n(expr, typeFactory.createMapType(expr.key().accept(this), expr.value().accept(this)));
    }

    private boolean n(NullableType type) {
      return type.nullable();
    }

    private RelDataType t(boolean nullable, SqlTypeName typeName, Integer... props) {
      final RelDataType baseType = switch(props.length) {
        case 0 -> typeFactory.createSqlType(typeName);
        case 1 -> typeFactory.createSqlType(typeName, props[0]);
        case 2 -> typeFactory.createSqlType(typeName, props[0], props[1]);
        default -> throw new IllegalArgumentException();
      };

      return typeFactory.createTypeWithNullability(baseType, nullable);
    }

    private RelDataType n(Type substraitType, RelDataType type) {
      return typeFactory.createTypeWithNullability(type, n(substraitType));
    }
  }

}
