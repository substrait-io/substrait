package io.substrait.relation;

public interface RelVisitor<OUTPUT, EXCEPTION extends Exception> {
  OUTPUT visit(Aggregate aggregate) throws EXCEPTION;
  OUTPUT visit(EmptyScan emptyScan) throws EXCEPTION;
  OUTPUT visit(Fetch fetch) throws EXCEPTION;
  OUTPUT visit(Filter filter) throws EXCEPTION;
  OUTPUT visit(Join join) throws EXCEPTION;
  OUTPUT visit(NamedScan namedScan) throws EXCEPTION;
  OUTPUT visit(Project project) throws EXCEPTION;
  OUTPUT visit(Sort sort) throws EXCEPTION;
  OUTPUT visit(VirtualTableScan virtualTableScan) throws EXCEPTION;
}
