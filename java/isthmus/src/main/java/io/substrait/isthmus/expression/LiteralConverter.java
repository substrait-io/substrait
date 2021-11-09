package io.substrait.isthmus.expression;

import com.google.protobuf.ByteString;
import io.substrait.expression.Expression;
import io.substrait.expression.ExpressionCreator;
import io.substrait.isthmus.TypeConverter;
import io.substrait.type.Type;

import org.apache.calcite.avatica.util.TimeUnitRange;
import org.apache.calcite.rel.type.RelDataType;
import org.apache.calcite.rex.RexLiteral;
import org.apache.calcite.util.*;

import java.math.BigDecimal;
import java.math.RoundingMode;
import java.time.*;
import java.time.format.DateTimeFormatter;
import java.time.format.DateTimeFormatterBuilder;
import java.util.List;
import java.util.concurrent.TimeUnit;

import static java.time.temporal.ChronoField.*;
import static io.substrait.expression.ExpressionCreator.*;
import static java.util.concurrent.TimeUnit.NANOSECONDS;

public class LiteralConverter {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(LiteralConverter.class);

  private static final long MICROS_IN_DAY = TimeUnit.DAYS.toMicros(1);

  static final DateTimeFormatter CALCITE_LOCAL_DATE_FORMATTER = DateTimeFormatter.ISO_LOCAL_DATE;
  static final DateTimeFormatter CALCITE_LOCAL_TIME_FORMATTER = new DateTimeFormatterBuilder()
      .appendValue(HOUR_OF_DAY, 2)
      .appendLiteral(':')
      .appendValue(MINUTE_OF_HOUR, 2)
      .appendLiteral(':')
      .appendValue(SECOND_OF_MINUTE, 2)
      .optionalStart()
      .appendFraction(NANO_OF_SECOND, 0, 9, true)
      .toFormatter();
  private static final DateTimeFormatter CALCITE_LOCAL_DATETIME_FORMATTER = new DateTimeFormatterBuilder()
      .parseCaseInsensitive()
      .append(CALCITE_LOCAL_DATE_FORMATTER)
      .appendLiteral(' ')
      .append(CALCITE_LOCAL_TIME_FORMATTER)
      .toFormatter();

  private static final DateTimeFormatter CALCITE_TIMESTAMP_WITH_ZONE_FORMATTER = new DateTimeFormatterBuilder()
      .parseCaseInsensitive()
      .append(CALCITE_LOCAL_DATE_FORMATTER)
      .appendLiteral(' ')
      .append(CALCITE_LOCAL_TIME_FORMATTER)
      .appendLiteral(' ')
      .appendZoneId()
      .toFormatter();

  private static final ZoneOffset SYSTEM_TIMEZONE = OffsetDateTime.now(ZoneId.systemDefault()).getOffset();

  private Expression nullOf(RexLiteral literal) {
    return null;
  }

  private static BigDecimal i(RexLiteral literal ) {
    return bd(literal).setScale(0, RoundingMode.HALF_UP);
  }

  private static String s(RexLiteral literal) {
    return ((NlsString)literal.getValue()).getValue();
  }

  private static BigDecimal bd(RexLiteral literal) {
    return (BigDecimal) literal.getValue();
  }

  public static Expression.Literal convert(RexLiteral literal) {
    // convert type first to guarantee we can handle the value.
    final Type type = TypeConverter.convert(literal.getType());
    final boolean n = type.nullable();

    if (literal.isNull()) {
      return typedNull(type);
    }

    return switch(literal.getType().getSqlTypeName()) {
      case TINYINT -> i8(n, i(literal).intValue());
      case SMALLINT -> i16(n, i(literal).intValue());
      case INTEGER -> i32(n, i(literal).intValue());
      case BIGINT -> i64(n, i(literal).longValue());
      case BOOLEAN -> bool(n, (Boolean) literal.getValue());
      case CHAR -> {
        var val = literal.getValue();
        yield switch(val) {
          case NlsString nls -> fixedChar(n, nls.getValue());
          default -> throw new UnsupportedOperationException("Unable to handle char type: " + val);
        };
      }
      case DOUBLE -> fp64(n, bd(literal).doubleValue());
      case FLOAT -> fp32(n, bd(literal).floatValue());

      case DECIMAL ->  {
        BigDecimal bd = bd(literal);
        yield decimal(n, bd, literal.getType().getPrecision(), literal.getType().getScale());
      }
      case VARCHAR -> {
        if (literal.getType().getPrecision() == RelDataType.PRECISION_NOT_SPECIFIED) {
          yield string(n, s(literal));
        }

        yield varChar(n, s(literal), literal.getType().getPrecision());

      }
      case BINARY -> fixedBinary(n, ByteString.copyFrom(padRightIfNeeded(literal.getValueAs(org.apache.calcite.avatica.util.ByteString.class), literal.getType().getPrecision())));
      case VARBINARY -> binary(n, ByteString.copyFrom(literal.getValueAs(byte[].class)));
      case SYMBOL -> {
        Object value = literal.getValue();
        yield switch(value) {
          case NlsString s -> string(n, s.getValue());
          case TimeUnitRange tur -> string(n, tur.name());
          default -> throw new UnsupportedOperationException("Unable to handle symbol: " + value);
        };
      }
      case DATE -> {
        DateString date = literal.getValueAs(DateString.class);
        LocalDate localDate = LocalDate.parse(date.toString(), CALCITE_LOCAL_DATE_FORMATTER);
        yield ExpressionCreator.date(n, (int) localDate.toEpochDay());
      }
      case TIME -> {
        TimeString time = literal.getValueAs(TimeString.class);
        LocalTime localTime = LocalTime.parse(time.toString(), CALCITE_LOCAL_TIME_FORMATTER);
        yield time(n, NANOSECONDS.toMicros(localTime.toNanoOfDay()));
      }

      case TIMESTAMP,TIMESTAMP_WITH_LOCAL_TIME_ZONE -> {
        TimestampString timestamp = literal.getValueAs(TimestampString.class);
        LocalDateTime ldt = LocalDateTime.parse(timestamp.toString(), CALCITE_LOCAL_DATETIME_FORMATTER);
        yield timestamp(n, ldt);
      }

      case INTERVAL_YEAR -> {
        var intervalLength = literal.getValueAs(BigDecimal.class).longValue();
        var years = intervalLength/12;
        var months = intervalLength - years * 12;
        yield intervalYear(n, (int) years, (int) months);
      }
      case INTERVAL_MONTH -> {
        var months = literal.getValueAs(BigDecimal.class).longValue();
        yield intervalYear(n, 0, (int) months);
      }

      case INTERVAL_YEAR_MONTH -> {
        throw new UnsupportedOperationException("Need to implement IntervalYear");
      }

      case INTERVAL_DAY -> {
        // we need to convert to microseconds.
        int precision = literal.getType().getPrecision();
        var intervalLength = literal.getValueAs(BigDecimal.class).longValue();
        var adjustedLength = precision > 6 ?
            intervalLength/((int)Math.pow(10,precision - 6))
            : intervalLength * ((int)Math.pow(10,6 - precision));
        var days = adjustedLength/MICROS_IN_DAY;
        var microseconds = adjustedLength - days * MICROS_IN_DAY;
        yield intervalDay(n, (int) days, (int) microseconds);
      }

      case INTERVAL_DAY_HOUR, INTERVAL_DAY_MINUTE, INTERVAL_DAY_SECOND, INTERVAL_HOUR, INTERVAL_HOUR_MINUTE,
      INTERVAL_HOUR_SECOND, INTERVAL_MINUTE, INTERVAL_MINUTE_SECOND, INTERVAL_SECOND -> {
        throw new UnsupportedOperationException("Need to implement IntervalDay");
      }

      case ROW -> {
        List<RexLiteral> literals = (List<RexLiteral>) literal.getValue();
        yield struct(n, literals.stream().map(LiteralConverter::convert).toList());
      }

      case ARRAY -> {
        List<RexLiteral> literals = (List<RexLiteral>) literal.getValue();
        yield list(n, literals.stream().map(LiteralConverter::convert).toList());
      }

      default ->
        throw new UnsupportedOperationException(String.format("Unable to convert the value of %s of type %s to a literal.", literal, literal.getType().getSqlTypeName()));
    };
  }

  public static byte[] padRightIfNeeded(org.apache.calcite.avatica.util.ByteString bytes, int length) {
    return padRightIfNeeded(bytes.getBytes(), length);
  }

  public static byte[] padRightIfNeeded(byte[] value, int length) {

    if (length < value.length) {
      throw new IllegalArgumentException("Byte values should either be at or below the expected length.");
    }

    if(length == value.length) {
      return value;
    }

    byte[] newArray = new byte[length];
    System.arraycopy(value, 0, newArray, 0, value.length);
    return newArray;
  }


}
