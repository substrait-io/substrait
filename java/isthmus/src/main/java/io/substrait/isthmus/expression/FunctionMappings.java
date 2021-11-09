package io.substrait.isthmus.expression;

import com.google.common.collect.ImmutableList;
import org.apache.calcite.sql.SqlOperator;
import org.apache.calcite.sql.fun.SqlStdOperatorTable;

import java.util.Locale;

public class FunctionMappings {
  // Static list of signature mapping between Calcite SQL operators and Substrait base function names.
  public static final ImmutableList<Sig> SCALAR_SIGS;
  public static final ImmutableList<Sig> AGGREGATE_SIGS;

  static {
    SCALAR_SIGS = ImmutableList.<Sig>builder().add(
        s(SqlStdOperatorTable.PLUS, "add"),
        s(SqlStdOperatorTable.MINUS, "subtract"),
        s(SqlStdOperatorTable.MULTIPLY, "multiply"),
        s(SqlStdOperatorTable.DIVIDE, "divide"),
        s(SqlStdOperatorTable.AND),
        s(SqlStdOperatorTable.OR),
        s(SqlStdOperatorTable.NOT),
        s(SqlStdOperatorTable.LESS_THAN, "lt"),
        s(SqlStdOperatorTable.LESS_THAN_OR_EQUAL, "lte"),
        s(SqlStdOperatorTable.GREATER_THAN, "gt"),
        s(SqlStdOperatorTable.GREATER_THAN_OR_EQUAL, "gte"),
        s(SqlStdOperatorTable.EQUALS, "equal"),
        s(SqlStdOperatorTable.BIT_XOR, "xor"),
        s(SqlStdOperatorTable.NOT_EQUALS, "not_equal"),
        s(SqlStdOperatorTable.MINUS_DATE, "subtract"),
        s(SqlStdOperatorTable.DATETIME_PLUS, "add"),
        s(SqlStdOperatorTable.LIKE)
    ).build();

    AGGREGATE_SIGS = ImmutableList.<Sig>builder().add(
        s(SqlStdOperatorTable.SUM, "sum"),
        s(SqlStdOperatorTable.COUNT, "count"),
        s(SqlStdOperatorTable.AVG, "avg")
    ).build();
  }

  public static void main(String[] args) {
    SCALAR_SIGS.forEach(System.out::println);
  }
  public static Sig s(SqlOperator operator, String substraitName) {
    return new Sig(operator, substraitName.toLowerCase(Locale.ROOT));
  }

  public static Sig s(SqlOperator operator) {
    return s(operator, operator.getName().toLowerCase(Locale.ROOT));
  }

  record Sig(SqlOperator operator, String name) {}
}
