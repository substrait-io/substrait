package io.substrait.isthmus.expression;

import org.apache.calcite.sql.SqlOperator;
import org.apache.calcite.sql.fun.SqlStdOperatorTable;

public class FunctionMatch {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(FunctionMatch.class);

  public static void load() {

    Matcher m = new Matcher() {


      {


        m("add",SqlStdOperatorTable.SUM);
        m("avg",SqlStdOperatorTable.AVG);
        m("count",SqlStdOperatorTable.COUNT);
        m("+",SqlStdOperatorTable.PLUS);
        m("+",SqlStdOperatorTable.DATETIME_PLUS);
        m("-",SqlStdOperatorTable.MINUS);
        m("*",SqlStdOperatorTable.MULTIPLY);
        m("/",SqlStdOperatorTable.DIVIDE);
      }
    };
  }

  static class Matcher {



    void m(String functionName, SqlOperator calciteOperator) {

    }
  }

}
