// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validating expressions.

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::string_util;
use crate::string_util::Describe;
use std::sync::Arc;

pub mod conditionals;
pub mod functions;
pub mod literals;
pub mod misc;
pub mod references;
pub mod subqueries;

/// Description of an expression.
#[derive(Clone)]
pub enum Expression {
    /// Used for unknown expression types.
    Unresolved,

    /// Used for literals.
    Literal(literals::Literal),

    /// Used for references.
    Reference(Box<references::Reference>),

    /// Used for function calls and conditionals (which, really, are just
    /// builtin function calls).
    Function(String, Vec<Expression>),

    /// Used to represent the values of a MultiOrList.
    Tuple(Vec<Expression>),

    /// Used for type casts.
    Cast(Arc<data_type::DataType>, Box<Expression>),

    /// Used for function option enum variants. Note that these aren't normal
    /// expressions, as they have no associated type. See FIXME at the bottom
    /// of this file.
    EnumVariant(Option<String>),

    /// Used for subqueries. These don't carry any description with them,
    /// because the structure is so extensive.
    SubQuery,
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Unresolved
    }
}

impl From<literals::Literal> for Expression {
    fn from(l: literals::Literal) -> Self {
        Expression::Literal(l)
    }
}

impl From<references::Reference> for Expression {
    fn from(r: references::Reference) -> Self {
        Expression::Reference(Box::new(r))
    }
}

impl Describe for Expression {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: string_util::Limit,
    ) -> std::fmt::Result {
        match self {
            Expression::Unresolved => write!(f, "?"),
            Expression::Literal(x) => x.describe(f, limit),
            Expression::Reference(x) => x.describe(f, limit),
            Expression::Function(name, args) => {
                let (name_limit, args_limit) = limit.split(name.len());
                string_util::describe_identifier(f, name, name_limit)?;
                write!(f, "(")?;
                string_util::describe_sequence(f, args, args_limit, 20, |f, expr, _, limit| {
                    expr.describe(f, limit)
                })?;
                write!(f, ")")
            }
            Expression::Tuple(items) => {
                write!(f, "(")?;
                string_util::describe_sequence(f, items, limit, 20, |f, expr, _, limit| {
                    expr.describe(f, limit)
                })?;
                write!(f, ")")
            }
            Expression::Cast(data_type, expression) => {
                let (type_limit, expr_limit) = limit.split(10);
                write!(f, "(")?;
                data_type.describe(f, type_limit)?;
                write!(f, ")(")?;
                expression.describe(f, expr_limit)?;
                write!(f, ")")
            }
            Expression::EnumVariant(Some(x)) => string_util::describe_identifier(f, x, limit),
            Expression::EnumVariant(None) => write!(f, "-"),
            Expression::SubQuery => write!(f, "subquery(..)"),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl Expression {
    /// Shorthand for a new null literal.
    pub fn new_null(data_type: Arc<data_type::DataType>) -> Expression {
        literals::Literal::new_null(data_type).into()
    }
}

/// Parse an expression type. Returns a description of said expression.
fn parse_expression_type(
    x: &substrait::expression::RexType,
    y: &mut context::Context,
    enum_allowed: bool,
) -> diagnostic::Result<Expression> {
    match x {
        substrait::expression::RexType::Literal(x) => {
            literals::parse_literal(x, y).map(Expression::from)
        }
        substrait::expression::RexType::Selection(x) => {
            references::parse_field_reference(x.as_ref(), y).map(Expression::from)
        }
        substrait::expression::RexType::ScalarFunction(x) => functions::parse_scalar_function(x, y),
        substrait::expression::RexType::WindowFunction(x) => functions::parse_window_function(x, y),
        substrait::expression::RexType::IfThen(x) => conditionals::parse_if_then(x.as_ref(), y),
        substrait::expression::RexType::SwitchExpression(x) => {
            conditionals::parse_switch(x.as_ref(), y)
        }
        substrait::expression::RexType::SingularOrList(x) => {
            conditionals::parse_singular_or_list(x.as_ref(), y)
        }
        substrait::expression::RexType::MultiOrList(x) => conditionals::parse_multi_or_list(x, y),
        substrait::expression::RexType::Enum(x) => {
            if !enum_allowed {
                diagnostic!(
                    y,
                    Error,
                    IllegalValue,
                    "function option enum variants are not allowed here"
                );
            }
            misc::parse_enum(x, y)
        }
        substrait::expression::RexType::Cast(x) => misc::parse_cast(x.as_ref(), y),
        substrait::expression::RexType::Subquery(x) => {
            subqueries::parse_subquery(x.as_ref(), y)?;
            Ok(Expression::SubQuery)
        }
    }
}

/// Parse an expression. Returns a description of said expression.
fn parse_expression_internal(
    x: &substrait::Expression,
    y: &mut context::Context,
    enum_allowed: bool,
) -> diagnostic::Result<Expression> {
    // Parse the expression.
    let (n, e) = proto_required_field!(x, y, rex_type, parse_expression_type, enum_allowed);
    let expression = e.unwrap_or_default();
    let data_type = n.data_type();

    // Describe node.
    y.set_data_type(data_type);
    describe!(y, Expression, "{}", expression);
    summary!(y, "Expression: {:#}", expression);
    Ok(expression)
}

/// Parse a regular expression (anything except a function option enum
/// variant). Returns a description of said expression.
pub fn parse_expression(
    x: &substrait::Expression,
    y: &mut context::Context,
) -> diagnostic::Result<Expression> {
    parse_expression_internal(x, y, false)
}

/// Parse a predicate expression (a normal expression that yields a boolean).
/// Returns a description of said expression.
pub fn parse_predicate(
    x: &substrait::Expression,
    y: &mut context::Context,
) -> diagnostic::Result<Expression> {
    let expression = parse_expression_internal(x, y, false)?;
    let data_type = y.data_type();
    if !matches!(
        data_type.class(),
        data_type::Class::Simple(data_type::Simple::Boolean) | data_type::Class::Unresolved(_)
    ) {
        diagnostic!(
            y,
            Error,
            TypeMismatch,
            "predicates must yield booleans, but found {}",
            data_type
        );
    }
    Ok(expression)
}

/// Parse a function argument, which can be an expression or an enum option.
fn parse_function_argument(
    x: &substrait::Expression,
    y: &mut context::Context,
) -> diagnostic::Result<Expression> {
    parse_expression_internal(x, y, true)
}

// FIXME: above should really be solved with a oneof, or better yet, by
// separating the options passed to a function from its arguments.
