use std::cmp::Ordering;

use seed::prelude::*;

use crate::model::{Model, ColumnVisibility, BooleanOp};
use crate::update::Msg;

pub fn view(model: &Model) -> impl View<Msg> {
    div![
        attrs! {At::Id => "Main"},
        div![
            attrs! {At::Id => "Controls"},
            filter_controls(model),
            sort_controls(model),
            column_controls(model)
        ],
        item_table(model)
    ]
}

// Components

fn filter_controls(model: &Model) -> Node<Msg> {
    div![
        attrs! {At::Id => "Filters"; At::Class => "ControlView"},
        h2!["Filter"],
        div![
            attrs! {At::Id => "FilterList"; At::Class => "ControlList"},
            model
                .filters
                .iter()
                .enumerate()
                .map(|(i, _)| filter_control(model, i))
                .collect::<Vec<Node<Msg>>>(),
            add_filter_button()
        ]
    ]
}

fn filter_control(model: &Model, index: usize) -> Node<Msg> {
    let filter = &model.filters[index];

    div![
        attrs! {At::Class => "Filter"},
        if index == 0 {
            select![
                attrs! {At::Class => "FilterBooleanOpInput";},
                input_ev(Ev::Input, move |value| Msg::ChangeFilterBooleanOp(index, value)),
                option![attrs! {At::Value => "And"; At::Selected => (filter.boolean_op == BooleanOp::And).as_at_value()}, ""],
                option![attrs! {At::Value => "AndNot"; At::Selected => (filter.boolean_op == BooleanOp::AndNot).as_at_value()}, "NOT"],
            ]
        } else {
            select![
                attrs! {At::Class => "FilterBooleanOpInput";},
                input_ev(Ev::Input, move |value| Msg::ChangeFilterBooleanOp(index, value)),
                option![attrs! {At::Value => "And"; At::Selected => (filter.boolean_op == BooleanOp::And).as_at_value()}, "AND"],
                option![attrs! {At::Value => "AndNot"; At::Selected => (filter.boolean_op == BooleanOp::AndNot).as_at_value()}, "AND NOT"],
                option![attrs! {At::Value => "Or"; At::Selected => (filter.boolean_op == BooleanOp::Or).as_at_value()}, "OR"],
                option![attrs! {At::Value => "OrNot"; At::Selected => (filter.boolean_op == BooleanOp::OrNot).as_at_value()}, "OR NOT"]
            ]
        },
        select![
            attrs! {At::Class => "FilterColumn"},
            input_ev(Ev::Input, move |value| Msg::ChangeFilterColumn(index, value)),
            option![attrs! {At::Value => ""; At::Selected => (filter.column == Option::None).as_at_value()}, ""],
            model
                .columns
                .iter()
                .enumerate()
                .map(|(i, c)| option![attrs! {At::Value => i; At::Selected => (filter.column == Option::Some(i)).as_at_value()}, c])
                .collect::<Vec<Node<Msg>>>()
        ],
        input![
            attrs! {At::Class => "FilterValue"; At::Value => filter.value},
            input_ev(Ev::Input, move |value| Msg::ChangeFilterValue(index, value))
        ],
        button![
            simple_ev(Ev::Click, Msg::RemoveFilter(index)),
            "-"
        ]
    ]
}

fn add_filter_button() -> Node<Msg> {
    button![
        simple_ev(Ev::Click, Msg::AddFilter),
        "+ Add Filter"
    ]
}

fn sort_controls(model: &Model) -> Node<Msg> {
    div![
        attrs! {At::Id => "Sort", At::Class => "ControlView"},
        h2!["Sort"],
        div![
            attrs! {At::Class => "ControlList"},
            model
                .sort_columns
                .iter()
                .enumerate()
                .map(|(i, _)| sort_control(model, i))
                .collect::<Vec<Node<Msg>>>(),
            add_sort_button()
        ]
    ]
}

fn sort_control(model: &Model, index: usize) -> Node<Msg> {
    let column_index = model.sort_columns[index];

    div![
        select![
            input_ev(Ev::Input, move |value| Msg::ChangeSortColumn(index, value)),
            option![attrs! {At::Value => ""; At::Selected => (column_index == Option::None).as_at_value()}, ""],
            model
                .columns
                .iter()
                .enumerate()
                .map(|(i, c)| option![attrs! {At::Value => i; At::Selected => (column_index == Option::Some(i)).as_at_value()}, c])
                .collect::<Vec<Node<Msg>>>()
        ],
        button![
            simple_ev(Ev::Click, Msg::RemoveSortColumn(index)),
            "-"
        ]
    ]
}

fn add_sort_button() -> Node<Msg> {
    button![
        simple_ev(Ev::Click, Msg::AddSortColumn),
        "+ Add Sort"
    ]
}

fn column_controls(model: &Model) -> Node<Msg> {
    div![
        attrs! {At::Id => "Columns"; At::Class => "ControlView"},
        h2!["Columns"],
        div![
            attrs! {At::Id => "ColumnList"; At::Class => "ControlList"},
            all_column_control(),
            model
                .columns
                .iter()
                .enumerate()
                .map(|(i, _)| column_control(model, i))
                .collect::<Vec<Node<Msg>>>()
        ]
    ]
}

fn all_column_control() -> Node<Msg> {
    let input_id = "column_visibility_all".to_string();

    div![
        attrs! {At::Class => "ColumnControl"},
        label![
            attrs! {At::Class => "AllColumnControlLabel"; At::For => input_id},
            "All"
        ],
        select![
            attrs! {At::Id => input_id; At::Class => "ColumnControlInput";},
            input_ev(Ev::Input, move |value| Msg::ChangeAllColumnVisibility(value)),
            option![attrs! {At::Value => ""; At::Selected => true.as_at_value()}, ""],
            option![attrs! {At::Value => "Auto";}, "Auto"],
            option![attrs! {At::Value => "Shown";}, "Shown"],
            option![attrs! {At::Value => "Hidden";}, "Hidden"]
        ]
    ]
}

fn column_control(model: &Model, column_index: usize) -> Node<Msg> {
    let input_id = format!("column_visibility_{}", column_index);
    let current_value = model.column_visibility[column_index];

    div![
        attrs! {At::Class => "ColumnControl"},
        label![
            attrs! {At::Class => "ColumnControlLabel"; At::For => input_id},
            model.columns[column_index]
        ],
        select![
            attrs! {At::Id => input_id; At::Class => "ColumnControlInput";},
            input_ev(Ev::Input, move |value| Msg::ChangeColumnVisibility(column_index, value)),
            option![attrs! {At::Value => "Auto"; At::Selected => (current_value == ColumnVisibility::Auto).as_at_value()}, "Auto"],
            option![attrs! {At::Value => "Shown"; At::Selected => (current_value == ColumnVisibility::Shown).as_at_value()}, "Shown"],
            option![attrs! {At::Value => "Hidden"; At::Selected => (current_value == ColumnVisibility::Hidden).as_at_value()}, "Hidden"]
        ]
    ]
}

fn item_table(model: &Model) -> Node<Msg> {
    let mut sorted_items = model
        .items
        .iter()
        .filter(|r| apply_filters(model, r))
        .collect::<Vec<&Vec<String>>>();
    sorted_items.sort_by(|a, b| sort_items(model, a, b));
    let actual_column_visibility = compute_column_visibility(model, &sorted_items);

    div![
        attrs! {At::Id => "Data", At::Class => "DataView"},
        format!("{} Items Shown", sorted_items.len()),
        table![
            thead![header_row(model, &actual_column_visibility)],
            tbody![sorted_items
                .iter()
                .map(|item| item_row(&actual_column_visibility, item))
                .collect::<Vec<Node<Msg>>>()]
        ]
    ]
}

fn header_row(model: &Model, column_visibility: &Vec<bool>) -> Node<Msg> {
    tr![model
        .columns
        .iter()
        .enumerate()
        .filter(|(i, _)| column_visibility[*i])
        .map(|(_, c)| th![c])
        .collect::<Vec<Node<Msg>>>()]
}

fn item_row(column_visibility: &Vec<bool>, item: &Vec<String>) -> Node<Msg> {
    tr![item
        .iter()
        .enumerate()
        .filter(|(i, _)| column_visibility[*i])
        .map(|(_, c)| td![div![attrs! {At::Class => "ItemValue"}, c]])
        .collect::<Vec<Node<Msg>>>()]
}

// Helper Functions

fn apply_filters(model: &Model, item: &[String]) -> bool {
    let mut current_filter_result: Option<bool> = Option::None;

    for filter in model.filters.clone() {
        if filter.value.is_empty() {
            continue;
        }

        if let Some(c) = filter.column {
            match filter.boolean_op {
                BooleanOp::And => {
                    if current_filter_result == Option::Some(false) {
                        break;
                    } else {
                        current_filter_result = Option::Some(item[c].to_lowercase().contains(&filter.value.to_lowercase()));
                    }
                }
                BooleanOp::AndNot => {
                    if current_filter_result == Option::Some(false) {
                        break;
                    } else {
                        current_filter_result = Option::Some(!item[c].to_lowercase().contains(&filter.value.to_lowercase()));
                    }
                }
                BooleanOp::Or => {
                    if current_filter_result == Option::Some(false) {
                        current_filter_result = Option::Some(item[c].to_lowercase().contains(&filter.value.to_lowercase()));
                    }
                }
                BooleanOp::OrNot => {
                    if current_filter_result == Option::Some(false) {
                        current_filter_result = Option::Some(!item[c].to_lowercase().contains(&filter.value.to_lowercase()));
                    }
                }
            }
        }
    }

    match current_filter_result {
        None => true,
        Some(result) => result
    }
}

fn sort_items(model: &Model, item_a: &[String], item_b: &[String]) -> Ordering {
    for column in &model.sort_columns {
        if let Some(i) = column {
            if item_a[*i].is_empty() && !item_b[*i].is_empty() {
                return Ordering::Greater;
            } else if !item_a[*i].is_empty() && item_b[*i].is_empty() {
                return Ordering::Less;
            } else {
                let ordering = item_a[*i].to_lowercase().cmp(&item_b[*i].to_lowercase());
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
        } else {
            return Ordering::Equal;
        }
    }

    Ordering::Equal
}

fn compute_column_visibility(model: &Model, items: &Vec<&Vec<String>>) -> Vec<bool> {
    let mut actual_column_visibility = vec![false; model.columns.len()];
    for (column, vis_option) in model.column_visibility.iter().enumerate() {
        actual_column_visibility[column] = match vis_option {
            ColumnVisibility::Hidden => false,
            ColumnVisibility::Shown => true,
            ColumnVisibility::Auto => items.iter().any(|item| !item[column].is_empty())
        }
    }
    return actual_column_visibility;
}