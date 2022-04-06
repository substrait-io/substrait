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

// FIXME: what promotions are allowed and when? I saw Isthmus output an
// if/else with branches differing in nullability, and that makes sense to me
// as something to support. But on the other hand, explicit type casts for
// everything might be nicer for a machine format. Either way, I'm not sure
// the specification has anything to say about this?

/// Parse an if-then expression. Returns a description of said expression.
pub fn parse_if_then(
    x: &substrait::expression::IfThen,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    let mut return_type: Arc<data_type::DataType> = Arc::default();
    let mut args = vec![];

    // Handle branches.
    proto_required_repeated_field!(x, y, ifs, |x, y| {
        // Parse fields.
        let (n, e) = proto_required_field!(x, y, r#if, expressions::parse_predicate);
        let condition = e.unwrap_or_default();
        let condition_type = n.data_type();
        let (n, e) = proto_required_field!(x, y, then, expressions::parse_expression);
        let value = e.unwrap_or_default();
        let value_type = n.data_type();

        // Check that the type is the same for each branch.
        return_type = types::promote_and_assert_equal(
            y,
            &value_type,
            &return_type,
            "branches must yield the same type",
        );

        // Nulls in the condition are propagated to the output.
        // FIXME: I guess?
        if !condition_type.is_unresolved() && condition_type.nullable() {
            return_type = return_type.make_nullable();
        }

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
        return_type = types::promote_and_assert_equal(
            y,
            &n.data_type(),
            &return_type,
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
        "Selects the value corresponding to the first condition that yields \
        true. If none of the conditions yield true, return {}.",
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
    proto_required_repeated_field!(x, y, ifs, |x, y| {
        // Parse match field.
        let (n, e) = proto_required_field!(x, y, r#if, literals::parse_literal);
        let match_value = e.unwrap_or_default();

        // Check that the type is the same for each branch.
        match_type = types::promote_and_assert_equal(
            y,
            &n.data_type(),
            &match_type,
            "literal type must match switch expression",
        );

        // Parse value field.
        let (n, e) = proto_required_field!(x, y, then, expressions::parse_expression);
        let value = e.unwrap_or_default();

        // Check that the type is the same for each branch.
        return_type = types::promote_and_assert_equal(
            y,
            &n.data_type(),
            &return_type,
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
        return_type = types::promote_and_assert_equal(
            y,
            &n.data_type(),
            &return_type,
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
        "Selects the value corresponding to the switch case that matches {}. \
        If none of the cases match, return {}.",
        args.first().unwrap(),
        args.last().unwrap()
    );
    let expression = expressions::Expression::Function(String::from("switch"), args);
    describe!(y, Expression, "{}", expression);
    Ok(expression)
}

/// Parse a "singular or list", i.e. something of the form
/// `x in (a, ..., c)`.
pub fn parse_singular_or_list(
    x: &substrait::expression::SingularOrList,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    let mut args = vec![];

    // Parse value to match.
    let (n, e) = proto_boxed_required_field!(x, y, value, expressions::parse_expression);
    let match_type = n.data_type();
    args.push(e.unwrap_or_default());

    // Handle allowed values.
    proto_required_repeated_field!(x, y, options, |x, y| {
        let expression = expressions::parse_expression(x, y)?;
        let value_type = y.data_type();
        args.push(expression);

        // Check that the type is the same as the value.
        types::assert_equal(
            y,
            &value_type,
            &match_type,
            "option type must match value type",
        );

        Ok(())
    });

    // Describe node.
    y.set_data_type(data_type::DataType::new_predicate(false));
    summary!(
        y,
        "Returns true if and only if {} is equal to any of the options.",
        args.first().unwrap()
    );
    let expression = expressions::Expression::Function(String::from("match"), args);
    describe!(y, Expression, "{}", expression);
    Ok(expression)
}

/// Parse a "multi or list", i.e. something of the form
/// `(x, .., z) in ((ax, .., az), .., (cx, .., cz))`.
pub fn parse_multi_or_list(
    x: &substrait::expression::MultiOrList,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // FIXME: why is there not just an expression that forms a struct from a
    // number of expressions? Then this could go away. Alternatively, why does
    // SingularOrList also exist, when it's just the special case of this
    // expression for one-tuples? And why is it named this confusingly?
    // (a in b, contains(a, b), matches(a, b) etc. would all make more sense
    // to me... at least add a comment in the protobuf descriptions)

    let mut args = vec![];

    // Parse value to match.
    let (ns, es) = proto_required_repeated_field!(x, y, value, expressions::parse_expression);
    let match_types = ns.iter().map(|x| x.data_type()).collect::<Vec<_>>();
    args.push(expressions::Expression::Tuple(
        es.into_iter().map(|x| x.unwrap_or_default()).collect(),
    ));

    // Handle allowed values.
    proto_required_repeated_field!(x, y, options, |x, y| {
        let (ns, es) = proto_required_repeated_field!(x, y, fields, expressions::parse_expression);
        let value_types = ns.iter().map(|x| x.data_type()).collect::<Vec<_>>();
        args.push(expressions::Expression::Tuple(
            es.into_iter().map(|x| x.unwrap_or_default()).collect(),
        ));

        // Check that the type is the same as the value.
        if match_types.len() != value_types.len() {
            diagnostic!(
                y,
                Error,
                TypeMismatch,
                "option types must match value types: numbers of fields differ"
            )
        }
        for (index, (value_type, match_type)) in
            value_types.iter().zip(match_types.iter()).enumerate()
        {
            types::assert_equal(
                y,
                value_type,
                match_type,
                format!("option type must match value type for field {index}"),
            );
        }

        Ok(())
    });

    // Describe node.
    y.set_data_type(data_type::DataType::new_predicate(false));
    summary!(
        y,
        "Returns true if and only if {} is equal to any of the options.",
        args.first().unwrap()
    );
    let expression = expressions::Expression::Function(String::from("match"), args);
    describe!(y, Expression, "{}", expression);
    Ok(expression)
}
