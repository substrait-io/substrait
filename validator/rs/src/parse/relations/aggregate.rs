// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for aggregate relations.
//!
//! The aggregate operation groups input data on one or more sets of grouping
//! keys, calculating each measure for each combination of grouping key.
//!
//! See <https://substrait.io/relations/logical_relations/#aggregate-operation>

use std::collections::HashSet;
use std::sync::Arc;

use crate::input::proto::substrait;
use crate::output::comment;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::expressions;
use crate::parse::expressions::functions;

/// Type of output field.
enum FieldType {
    /// A field passed straight on from the input, but uniquified.
    GroupedField,

    /// Like GroupedField, but grouping sets exist that this field is not a
    /// part of. Null will be returned for such rows.
    NullableGroupedField,

    /// An aggregate function applied to the input rows that were combined for
    /// the current output row.
    Measure,

    /// The index of the grouping set that the result corresponds to.
    GroupingSetIndex,
}

/// A grouping or aggregate expression returned by the aggregate relation.
struct Field {
    /// Description of the grouping or aggregate expression.
    expression: expressions::Expression,

    /// Data type returned by the expression.
    data_type: Arc<data_type::DataType>,

    /// The type of field.
    field_type: FieldType,
}

/// Parse a measure.
fn parse_measure(
    x: &substrait::aggregate_rel::Measure,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse the aggregate function.
    let (n, e) = proto_required_field!(x, y, measure, functions::parse_aggregate_function);
    let data_type = n.data_type();
    let expression = e.unwrap_or_default();
    y.set_data_type(data_type);

    // Parse the filter and describe the node.
    if x.filter.is_some() {
        let filter = proto_required_field!(x, y, filter, expressions::parse_predicate)
            .1
            .unwrap_or_default();
        summary!(
            y,
            "Applies aggregate function {expression:#} to all rows for \
            which {filter:#} returns true."
        );
        let filtered_expression =
            expressions::Expression::Function(String::from("filter"), vec![filter, expression]);
        describe!(
            y,
            Expression,
            "Filtered aggregate function: {filtered_expression}"
        );
        Ok(filtered_expression)
    } else {
        summary!(y, "Applies aggregate function {expression:#} to all rows.");
        describe!(y, Expression, "Aggregate function: {expression}");
        Ok(expression)
    }
}

/// Parse aggregate relation.
pub fn parse_aggregate_rel(
    x: &substrait::AggregateRel,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // Parse input.
    let in_type = handle_rel_input!(x, y);

    // Set schema context for the grouping and measure expressions.
    y.set_schema(in_type);

    // Parse grouping sets.
    let mut grouping_set_expressions: Vec<substrait::Expression> = vec![];
    let mut fields = vec![];
    let mut sets = vec![];
    proto_repeated_field!(x, y, groupings, |x, y| {
        sets.push(vec![]);
        proto_required_repeated_field!(x, y, grouping_expressions, |x, y| {
            let result = expressions::parse_expression(x, y);

            // See if we parsed this expression before. If not, add it to the
            // field list. Return the index in the field list.
            let index = grouping_set_expressions
                .iter()
                .enumerate()
                .find(|(_, e)| e == &x)
                .map(|(i, _)| i)
                .unwrap_or_else(|| {
                    // Create new field.
                    grouping_set_expressions.push(x.clone());
                    fields.push(Field {
                        expression: result.as_ref().cloned().unwrap_or_default(),
                        data_type: y.data_type(),
                        field_type: FieldType::NullableGroupedField,
                    });

                    fields.len() - 1
                });

            // Add index of uniquified field to grouping set.
            sets.last_mut().unwrap().push(index);

            result
        });
        Ok(())
    });
    drop(grouping_set_expressions);
    let sets = sets;

    // Each field that is part of all sets will never be made nullable by the
    // aggregate relation, so its type does not need to be made nullable.
    let mut set_iter = sets.iter();
    if let Some(first_set) = set_iter.next() {
        let mut fields_in_all_sets = first_set.iter().cloned().collect::<HashSet<_>>();
        for set in set_iter {
            fields_in_all_sets = &fields_in_all_sets & &set.iter().cloned().collect::<HashSet<_>>();
        }
        for index in fields_in_all_sets {
            fields[index].field_type = FieldType::GroupedField;
        }
    }

    // Parse measures.
    proto_repeated_field!(x, y, measures, |x, y| {
        let result = parse_measure(x, y);
        fields.push(Field {
            expression: result.as_ref().cloned().unwrap_or_default(),
            data_type: y.data_type(),
            field_type: FieldType::Measure,
        });
        result
    });

    // The relation is invalid if no fields result from it.
    if fields.is_empty() {
        diagnostic!(
            y,
            Error,
            RelationInvalid,
            "aggregate relations must have at least one grouping expression or measure"
        );
    }

    // Add the column for the grouping set index.
    // FIXME: this field makes no sense for aggregate relations that only have
    // measures. It's also disputable whether it should exist when there is
    // only one grouping set.
    fields.push(Field {
        expression: expressions::Expression::Function(String::from("group_index"), vec![]),
        data_type: data_type::DataType::new_integer(false),
        field_type: FieldType::GroupingSetIndex,
    });
    let fields = fields;

    // Derive schema.
    y.set_schema(data_type::DataType::new_struct(
        fields.iter().map(|x| {
            if matches!(x.field_type, FieldType::NullableGroupedField) {
                x.data_type.make_nullable()
            } else {
                x.data_type.clone()
            }
        }),
        false,
    ));

    // Describe the relation.
    if x.groupings.is_empty() {
        describe!(y, Relation, "Aggregate");
        summary!(
            y,
            "This relation computes {} aggregate function(s) over all rows, \
            returning a single row.",
            x.measures.len()
        );
    } else if x.measures.is_empty() {
        describe!(y, Relation, "Group");
        summary!(
            y,
            "This relation groups rows from the input by the result of some \
            expression(s)."
        );
    } else {
        describe!(y, Relation, "Group & aggregate");
        summary!(
            y,
            "This relation groups rows from the input by the result of some \
            expression(s), and also compures {} aggregate function(s) over \
            each group.",
            x.measures.len()
        );
    }
    let mut comment = comment::Comment::new()
        .plain("The significance of the returned field(s) is:")
        .lo();
    for (index, field) in fields.iter().enumerate() {
        comment = comment.li().plain(match field.field_type {
            FieldType::GroupedField => format!(
                "Field {index}: value of grouping expression {:#}.",
                field.expression
            ),
            FieldType::NullableGroupedField => format!(
                "Field {index}: value of grouping expression {:#} if it is \
                part of the grouping set being returned, null otherwise.",
                field.expression
            ),
            FieldType::Measure => {
                if x.groupings.is_empty() {
                    format!(
                        "Field {index}: result of aggregate function {:#} \
                        applied to all input rows.",
                        field.expression
                    )
                } else {
                    format!(
                        "Field {index}: result of aggregate function {:#} \
                        applied to the rows from the current group.",
                        field.expression
                    )
                }
            }
            FieldType::GroupingSetIndex => {
                if x.groupings.is_empty() {
                    format!(
                        "Field {index}: undefined value, reserved for grouping \
                        set index."
                    )
                } else if x.groupings.len() == 1 {
                    format!(
                        "Field {index}: always zero, representing the index of the \
                        matched grouping set (of which there is only one here)."
                    )
                } else {
                    format!(
                        "Field {index}: integer between 0 and {} inclusive, \
                        representing the index of the matched grouping set.",
                        x.groupings.len() - 1
                    )
                }
            }
        });
    }
    y.push_summary(comment.lc());

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
