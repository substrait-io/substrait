package io.substrait.isthmus;

import org.apache.calcite.rel.RelNode;
import org.apache.calcite.rel.core.TableFunctionScan;
import org.apache.calcite.rel.core.TableScan;
import org.apache.calcite.rel.logical.*;

/**
 * A more generic version of RelShuttle that allows an alternative return value.
 */
public abstract class RelNodeVisitor<OUTPUT, EXCEPTION extends Throwable> {

  public OUTPUT visit(TableScan scan) throws EXCEPTION {
    return visitOther(scan);
  }
  public OUTPUT visit(TableFunctionScan scan) throws EXCEPTION {
    return visitOther(scan);
  }
  public OUTPUT visit(LogicalValues values) throws EXCEPTION {
    return visitOther(values);
  }
  public OUTPUT visit(LogicalFilter filter) throws EXCEPTION {
    return visitOther(filter);
  }
  public OUTPUT visit(LogicalCalc calc) throws EXCEPTION {
    return visitOther(calc);
  }
  public OUTPUT visit(LogicalProject project) throws EXCEPTION {
    return visitOther(project);
  }
  public OUTPUT visit(LogicalJoin join) throws EXCEPTION {
    return visitOther(join);
  }
  public OUTPUT visit(LogicalCorrelate correlate) throws EXCEPTION {
    return visitOther(correlate);
  }
  public OUTPUT visit(LogicalUnion union) throws EXCEPTION {
    return visitOther(union);
  }
  public OUTPUT visit(LogicalIntersect intersect) throws EXCEPTION {
    return visitOther(intersect);
  }
  public OUTPUT visit(LogicalMinus minus) throws EXCEPTION {
    return visitOther(minus);
  }
  public OUTPUT visit(LogicalAggregate aggregate) throws EXCEPTION {
    return visitOther(aggregate);
  }
  public OUTPUT visit(LogicalMatch match) throws EXCEPTION {
    return visitOther(match);
  }
  public OUTPUT visit(LogicalSort sort) throws EXCEPTION {
    return visitOther(sort);
  }
  public OUTPUT visit(LogicalExchange exchange) throws EXCEPTION {
    return visitOther(exchange);
  }
  public OUTPUT visit(LogicalTableModify modify) throws EXCEPTION {
    return visitOther(modify);
  }

  public abstract OUTPUT visitOther(RelNode other) throws EXCEPTION;

  protected RelNodeVisitor(){}

  /**
   * The method you call when you would normally call RelNode.accept(visitor). Instead call
   * RelVisitor.reverseAccept(RelNode) due to the lack of ability to extend base classes.
   */
  public final OUTPUT reverseAccept(RelNode node) throws EXCEPTION {
    if(node instanceof TableScan scan) {
      return this.visit(scan);
    } else if(node instanceof TableFunctionScan scan) {
      return this.visit(scan);
    } else if(node instanceof LogicalValues values) {
      return this.visit(values);
    } else if(node instanceof LogicalFilter filter) {
      return this.visit(filter);
    } else if(node instanceof LogicalCalc calc) {
      return this.visit(calc);
    } else if(node instanceof LogicalProject project) {
      return this.visit(project);
    } else if(node instanceof LogicalJoin join) {
      return this.visit(join);
    } else if(node instanceof LogicalCorrelate correlate) {
      return this.visit(correlate);
    } else if(node instanceof LogicalUnion union) {
      return this.visit(union);
    } else if(node instanceof LogicalIntersect intersect) {
      return this.visit(intersect);
    } else if(node instanceof LogicalMinus minus) {
      return this.visit(minus);
    } else if(node instanceof LogicalMatch match) {
      return this.visit(match);
    } else if(node instanceof LogicalSort sort) {
      return this.visit(sort);
    } else if(node instanceof LogicalExchange exchange) {
      return this.visit(exchange);
    } else if(node instanceof LogicalAggregate aggregate) {
      return this.visit(aggregate);
    } else if(node instanceof LogicalTableModify modify) {
      return this.visit(modify);
    } else {
      return this.visitOther(node);
    }

  }

}

