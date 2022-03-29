// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validating function calls.

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::output::extension;
use crate::output::tree;
use crate::parse::context;
use crate::parse::expressions;
use crate::parse::extensions;
use crate::parse::sorts;
use crate::parse::types;
use std::sync::Arc;

/// Matches a function call with its YAML definition, yielding its return type.
/// Yields an unresolved type if resolution fails.
pub fn check_function(
    y: &mut context::Context,
    _function: &extension::Function,
    _options: &[Option<String>],
    _arg_types: &[Arc<data_type::DataType>],
) -> Arc<data_type::DataType> {
    // TODO: check consistency of:
    //  - _function (function definition information from the YAML file);
    //  - _options: number of options passed to the function, and validity of
    //    their values;
    //  - _arg_types: whether an overload exists for this set of argument
    //    types;
    diagnostic!(
        y,
        Warning,
        NotYetImplemented,
        "matching function calls with their definitions"
    );
    Arc::default()
}

/// Parsing logic common to scalar and window functions.
fn parse_function(
    y: &mut context::Context,
    function: Option<Arc<extension::Reference<extension::Function>>>,
    arguments: (Vec<Arc<tree::Node>>, Vec<Option<expressions::Expression>>),
    return_type: Arc<data_type::DataType>,
) -> (Arc<data_type::DataType>, expressions::Expression) {
    // Determine the name of the function.
    let name = function
        .as_ref()
        .map(|x| x.name.to_string())
        .unwrap_or_else(|| String::from("?"));

    // Unpack the arguments into the function's enum options and regular
    // arguments.
    let mut opt_values = vec![];
    let mut opt_exprs = vec![];
    let mut arg_types = vec![];
    let mut arg_exprs = vec![];
    for (node, expr) in arguments
        .0
        .into_iter()
        .zip(arguments.1.into_iter().map(|x| x.unwrap_or_default()))
    {
        if let expressions::Expression::EnumVariant(x) = &expr {
            if opt_exprs.is_empty() && !arg_exprs.is_empty() {
                diagnostic!(
                    y,
                    Error,
                    IllegalValue,
                    "function option argument specified after first regular argument"
                );
            }
            opt_values.push(x.clone());
            opt_exprs.push(expr);
        } else {
            arg_types.push(node.data_type());
            arg_exprs.push(expr);
        }
    }
    opt_exprs.extend(arg_exprs.into_iter());
    let expression = expressions::Expression::Function(name, opt_exprs);
    let opt_values = opt_values;
    let arg_types = arg_types;

    // If the function was resolved, check whether it's valid.
    let return_type = if let Some(reference) = function {
        if let Some(function) = &reference.definition {
            let derived = check_function(y, function, &opt_values, &arg_types);
            types::assert_equal(
                y,
                &return_type,
                &derived,
                "specified return type must match derived",
            )
        } else {
            diagnostic!(
                y,
                Warning,
                ExpressionFunctionDefinitionUnavailable,
                "cannot check validity of call"
            );
            return_type
        }
    } else {
        return_type
    };

    (return_type, expression)
}

/// Parse a scalar function. Returns a description of the function call
/// expression.
pub fn parse_scalar_function(
    x: &substrait::expression::ScalarFunction,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse function information.
    let function = proto_primitive_field!(
        x,
        y,
        function_reference,
        extensions::simple::parse_function_reference
    )
    .1;
    let arguments = proto_repeated_field!(x, y, args, expressions::parse_function_argument);
    let return_type = proto_required_field!(x, y, output_type, types::parse_type)
        .0
        .data_type();

    // Check function information.
    let (return_type, expression) = parse_function(y, function, arguments, return_type);

    // Describe node.
    y.set_data_type(return_type);
    describe!(y, Expression, "{}", expression);
    summary!(y, "Scalar function call: {:#}", expression);
    Ok(expression)
}

/// Parse a window function bound.
fn parse_bound(
    _x: &substrait::expression::window_function::Bound,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // TODO: check window function bound.
    // FIXME: I have no idea what these bounds signify. The spec doesn't
    // seem to specify.
    diagnostic!(
        y,
        Warning,
        NotYetImplemented,
        "validation of window function bounds"
    );
    Ok(())
}

/// Parse a window function. Returns a description of the function call
/// expression.
pub fn parse_window_function(
    x: &substrait::expression::WindowFunction,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse function information.
    let function = proto_primitive_field!(
        x,
        y,
        function_reference,
        extensions::simple::parse_function_reference
    )
    .1;
    let arguments = proto_repeated_field!(x, y, args, expressions::parse_function_argument);
    let return_type = proto_required_field!(x, y, output_type, types::parse_type)
        .0
        .data_type();

    // Check function information.
    let (return_type, expression) = parse_function(y, function, arguments, return_type);

    // Parse modifiers.
    proto_repeated_field!(x, y, partitions, expressions::parse_expression);
    proto_repeated_field!(x, y, sorts, sorts::parse_sort_field);
    proto_field!(x, y, upper_bound, parse_bound);
    proto_field!(x, y, lower_bound, parse_bound);
    proto_enum_field!(x, y, phase, substrait::AggregationPhase);

    // TODO: check window function configuration.
    // FIXME: I have no idea what these partitions signify. The spec doesn't
    // seem to specify.
    if !x.partitions.is_empty() {
        diagnostic!(
            y,
            Warning,
            NotYetImplemented,
            "validation of partitions field"
        );
    }

    // Describe node.
    y.set_data_type(return_type);
    describe!(y, Expression, "{}", expression);
    summary!(y, "Window function call: {:#}", expression);
    Ok(expression)
}

/// Parse an aggregate function. Returns a description of the function call
/// expression.
pub fn parse_aggregate_function(
    x: &substrait::AggregateFunction,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse function information.
    let function = proto_primitive_field!(
        x,
        y,
        function_reference,
        extensions::simple::parse_function_reference
    )
    .1;
    let arguments = proto_repeated_field!(x, y, args, expressions::parse_function_argument);
    let return_type = proto_required_field!(x, y, output_type, types::parse_type)
        .0
        .data_type();

    // Check function information.
    let (return_type, expression) = parse_function(y, function, arguments, return_type);

    // Parse modifiers.
    proto_repeated_field!(x, y, sorts, sorts::parse_sort_field);
    proto_enum_field!(x, y, phase, substrait::AggregationPhase);

    // Describe node.
    y.set_data_type(return_type);
    describe!(y, Expression, "{}", expression);
    summary!(y, "Aggregate function call: {:#}", expression);
    Ok(expression)
}
