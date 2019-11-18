use std::cmp::Ordering;

use seed::prelude::*;

use crate::model::Model;
use crate::model::ColumnVisibility;
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
        model
            .filters
            .iter()
            .enumerate()
            .map(|(i, _)| filter_control(model, i))
            .collect::<Vec<Node<Msg>>>()
    ]
}

fn filter_control(model: &Model, index: usize) -> Node<Msg> {
    div![
        attrs! {At::Class => "Filter"},
        select![
            attrs! {At::Class => "FilterColumn"; At::Value => ""},
            input_ev(Ev::Input, move |value| Msg::ChangeFilterColumn(
                index, value
            )),
            option![attrs! {At::Value => ""}, ""],
            model
                .columns
                .iter()
                .enumerate()
                .map(|(i, c)| option![attrs! {At::Value => i}, c])
                .collect::<Vec<Node<Msg>>>()
        ],
        input![
            attrs! {At::Class => "FilterValue"; At::Value => model.filters[index].1},
            input_ev(Ev::Input, move |value| Msg::ChangeFilterValue(index, value))
        ]
    ]
}

fn column_controls(model: &Model) -> Node<Msg> {
    div![
        attrs! {At::Id => "Columns"; At::Class => "ControlView"},
        h2!["Columns"],
        div![
            attrs! {At::Id => "ColumnsList"},
            all_column_control(model),
            model
                .columns
                .iter()
                .enumerate()
                .map(|(i, _)| column_control(model, i))
                .collect::<Vec<Node<Msg>>>()
        ]
    ]
}

fn all_column_control(model: &Model) -> Node<Msg> {
    let input_id = "column_visibility_all".to_string();

    div![
        attrs! {At::Class => "ColumnControl"},
        label![
            attrs! {At::Class => "ColumnControlLabel"; At::For => input_id},
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

fn sort_controls(model: &Model) -> Node<Msg> {
    div![
        attrs! {At::Id => "Sort", At::Class => "ControlView"},
        h2!["Sort"],
        model
            .sort_columns
            .iter()
            .enumerate()
            .map(|(i, _)| div![select![
                attrs! {At::Value => ""},
                input_ev(Ev::Input, move |value| Msg::ChangeSortColumn(i, value)),
                option![attrs! {At::Value => ""}, ""],
                model
                    .columns
                    .iter()
                    .enumerate()
                    .map(|(i, c)| option![attrs! {At::Value => i}, c])
                    .collect::<Vec<Node<Msg>>>()
            ]])
            .collect::<Vec<Node<Msg>>>()
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
        table![
            thead![tr![model
                .columns
                .iter()
                .enumerate()
                .filter(|(i, _)| actual_column_visibility[*i])
                .map(|(_, c)| th![c])
                .collect::<Vec<Node<Msg>>>()]],
            tbody![sorted_items
                .iter()
                .map(|r| tr![r
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| actual_column_visibility[*i])
                    .map(|(_, c)| td![c])
                    .collect::<Vec<Node<Msg>>>()])
                .collect::<Vec<Node<Msg>>>()]
        ]
    ]
}

// Helper Functions

fn apply_filters(model: &Model, item: &[String]) -> bool {
    for (column, filter_string) in model.filters.clone() {
        if let Some(c) = column {
            if !item[c]
                .to_lowercase()
                .contains(&filter_string.to_lowercase())
            {
                return false;
            }
        }
    }

    true
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