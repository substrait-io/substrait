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
use crate::parse::types;
use std::sync::Arc;

/// Matches a function call with its YAML definition.
fn check_function(
    y: &mut context::Context,
    _function: &extension::Function,
    _options: &[Option<String>],
    _arg_types: &[Arc<data_type::DataType>],
    _return_type: &Arc<data_type::DataType>,
) {
    // TODO: check consistency of:
    //  - _function (function definition information from the YAML
    //    file);
    //  - _options: number of options passed to the function, and
    //    validity of their values;
    //  - _arg_types: whether an overload exists for this set of
    //    argument types;
    //  - _return_type: whether the return type in the plan matches
    //    above overload (type expression evaluation???).
    diagnostic!(
        y,
        Warning,
        NotYetImplemented,
        "matching function calls with their definitions"
    );
}

/// Parsing logic common to scalar and window functions.
fn parse_function(
    y: &mut context::Context,
    function: Option<Arc<extension::Reference<extension::Function>>>,
    arguments: (Vec<Arc<tree::Node>>, Vec<Option<expressions::Expression>>),
    return_type: Arc<data_type::DataType>,
) -> expressions::Expression {
    // Determine the name of the function.
    let name = function
        .as_ref()
        .map(|x| x.common.name.clone())
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
    if let Some(reference) = function {
        if let Some(function) = &reference.definition {
            check_function(y, function, &opt_values, &arg_types, &return_type);
        } else {
            diagnostic!(
                y,
                Warning,
                ExpressionFunctionDefinitionUnavailable,
                "cannot check validity of call"
            );
        }
    }

    expression
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
    let expression = parse_function(y, function, arguments, return_type);

    // Describe node.
    describe!(y, Expression, "{}", expression);
    summary!(y, "Scalar function call: {:#}", expression);
    Ok(expression)
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
    let expression = parse_function(y, function, arguments, return_type);

    // TODO: check window function configuration.

    // Describe node.
    describe!(y, Expression, "{}", expression);
    summary!(y, "Window function call: {:#}", expression);
    Ok(expression)
}
