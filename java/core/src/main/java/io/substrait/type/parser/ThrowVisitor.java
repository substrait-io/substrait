package io.substrait.type.parser;

import io.substrait.type.SubstraitTypeBaseVisitor;
import org.antlr.v4.runtime.tree.RuleNode;

class ThrowVisitor<T> extends SubstraitTypeBaseVisitor<T> {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(ThrowVisitor.class);

  @Override public T visitChildren(final RuleNode node) {
    throw new UnsupportedOperationException();
  }
}
