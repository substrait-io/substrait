// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validating conditional expression types.

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::expressions;
use crate::parse::expressions::literals;
use crate::parse::types;
use std::sync::Arc;

/// Parse an if-then expression. Returns a description of said expression.
pub fn parse_if_then(
    x: &substrait::expression::IfThen,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    let mut return_type: Arc<data_type::DataType> = Arc::default();
    let mut args = vec![];

    // Handle branches.
    proto_repeated_field!(x, y, ifs, |x, y| {
        // Parse fields.
        let condition = proto_required_field!(x, y, r#if, expressions::parse_predicate)
            .1
            .unwrap_or_default();
        let (n, e) = proto_required_field!(x, y, then, expressions::parse_expression);
        let value = e.unwrap_or_default();

        // Check that the type is the same for each branch.
        return_type = types::assert_equal(
            y,
            n.data_type(),
            return_type.clone(),
            "branches must yield the same type",
        );

        // Describe this branch.
        describe!(y, Misc, "If {} yield {}", &condition, &value);

        // Save to the "arguments" of the function we'll use to describe this
        // expression.
        args.push(condition);
        args.push(value);

        Ok(())
    });

    // Handle else branch.
    if x.r#else.is_some() {
        // Parse field.
        let (n, e) = proto_boxed_required_field!(x, y, r#else, expressions::parse_expression);
        let value = e.unwrap_or_default();

        // Check that the type is the same for each branch.
        return_type = types::assert_equal(
            y,
            n.data_type(),
            return_type.clone(),
            "branches must yield the same type",
        );

        // Save to the "arguments" of the function we'll use to describe this
        // expression.
        args.push(value);
    } else {
        // Allow missing else, making the type nullable.
        comment!(y, "Otherwise, yield null.");
        return_type = return_type.make_nullable();

        // Yield null for the else clause.
        args.push(expressions::Expression::new_null(return_type.clone()));
    }

    // Describe node.
    y.set_data_type(return_type);
    summary!(
        y,
        "Selects the value corresponding to the first condition that yields true."
    );
    summary!(
        y,
        "If none of the conditions yield true, return {}.",
        args.last().unwrap()
    );
    let expression = expressions::Expression::Function(String::from("if_then"), args);
    describe!(y, Expression, "{}", expression);
    Ok(expression)
}

/// Parse a switch expression. Returns a description of said expression.
pub fn parse_switch(
    x: &substrait::expression::SwitchExpression,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    let mut return_type: Arc<data_type::DataType> = Arc::default();
    let mut args = vec![];

    // Parse value to match.
    let (n, e) = proto_boxed_required_field!(x, y, r#match, expressions::parse_expression);
    let mut match_type = n.data_type();
    args.push(e.unwrap_or_default());

    // Handle branches.
    proto_repeated_field!(x, y, ifs, |x, y| {
        // Parse match field.
        let (n, e) = proto_required_field!(x, y, r#if, literals::parse_literal);
        let match_value = e.unwrap_or_default();

        // Check that the type is the same for each branch.
        match_type = types::assert_equal(
            y,
            n.data_type(),
            match_type.clone(),
            "literal type must match switch expression",
        );

        // Parse value field.
        let (n, e) = proto_required_field!(x, y, then, expressions::parse_expression);
        let value = e.unwrap_or_default();

        // Check that the type is the same for each branch.
        return_type = types::assert_equal(
            y,
            n.data_type(),
            return_type.clone(),
            "branches must yield the same type",
        );

        // Describe this branch.
        describe!(y, Misc, "If match == {} yield {}", &match_value, &value);

        // Save to the "arguments" of the function we'll use to describe this
        // expression.
        args.push(match_value.into());
        args.push(value);

        Ok(())
    });

    // Handle else branch.
    if x.r#else.is_some() {
        // Parse field.
        let (n, e) = proto_boxed_required_field!(x, y, r#else, expressions::parse_expression);
        let value = e.unwrap_or_default();

        // Check that the type is the same for each branch.
        return_type = types::assert_equal(
            y,
            n.data_type(),
            return_type.clone(),
            "branches must yield the same type",
        );

        // Save to the "arguments" of the function we'll use to describe this
        // expression.
        args.push(value);
    } else {
        // Allow missing else, making the type nullable.
        comment!(y, "Otherwise, yield null.");
        return_type = return_type.make_nullable();

        // Yield null for the else clause.
        args.push(expressions::Expression::new_null(return_type.clone()));
    }

    // Describe node.
    y.set_data_type(return_type);
    summary!(
        y,
        "Selects the value corresponding to the switch case that matches {}.",
        args.first().unwrap()
    );
    summary!(
        y,
        "If none of the cases match, return {}.",
        args.last().unwrap()
    );
    let expression = expressions::Expression::Function(String::from("switch"), args);
    describe!(y, Expression, "{}", expression);
    Ok(expression)
}

/// Parse a "singular or list", i.e. something of the form
/// `x in (a, ..., c)`.
pub fn parse_singular_or_list(
    _x: &substrait::expression::SingularOrList,
    _y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // TODO
    Ok(expressions::Expression::Function(
        String::from("in"),
        vec![],
    ))
}

/// Parse a "multi or list", i.e. something of the form
/// `(x, .., z) in ((ax, .., az), .., (cx, .., cz))`.
pub fn parse_multi_or_list(
    _x: &substrait::expression::MultiOrList,
    _y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // FIXME: why is there not just an expression that forms a struct from a
    // number of expressions? Then this could go away. Alternatively, why does
    // SingularOrList also exist, when it's just the special case of this
    // expression for one-tuples? And why is it named this confusingly?
    // (a in b, contains(a, b), matches(a, b) etc. would all make more sense
    // to me... at least add a comment in the protobuf descriptions)

    // TODO
    Ok(expressions::Expression::Function(
        String::from("in"),
        vec![],
    ))
}
