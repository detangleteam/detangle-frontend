use std::cmp::Ordering;

use seed::prelude::*;

use crate::model::Model;
use crate::update::Msg;

pub fn view(model: &Model) -> impl View<Msg> {
    div![
        attrs! {At::Id => "Main"},
        div![
            attrs! {At::Id => "Controls"},
            filter_view(model),
            sort_view(model),
            column_visibility_view(model)
        ],
        data_view(model)
    ]
}

fn filter_view(model: &Model) -> Node<Msg> {
    div![
        attrs! {At::Id => "Filters", At::Class => "ControlView"},
        h2!["Filter"],
        model
            .filters
            .iter()
            .enumerate()
            .map(|(i, _)| div![
                select![
                    attrs! {At::Value => ""},
                    input_ev(Ev::Input, move |value| Msg::ChangeFilterColumn(i, value)),
                    option![attrs! {At::Value => ""}, "--"],
                    model
                        .columns
                        .iter()
                        .enumerate()
                        .map(|(i, c)| option![attrs! {At::Value => i}, c])
                        .collect::<Vec<Node<Msg>>>()
                ],
                input![input_ev(Ev::Input, move |value| Msg::ChangeFilterValue(
                    i, value
                ))]
            ])
            .collect::<Vec<Node<Msg>>>()
    ]
}

fn column_visibility_view(model: &Model) -> Node<Msg> {
    div![
        attrs! {At::Id => "Columns", At::Class => "ControlView"},
        h2!["Columns"],
        div![
            attrs! {At::Id => "ColumnsList"},
            model
                .columns
                .iter()
                .enumerate()
                .map(|(i, c)| div![
                    label![c],
                    input![
                        attrs! {At::Type => "checkbox"; At::Checked => true},
                        input_ev(Ev::Input, move |value| Msg::ChangeColumnVisibility(
                            i, value
                        ))
                    ]
                ])
                .collect::<Vec<Node<Msg>>>()
        ]
    ]
}

fn sort_view(model: &Model) -> Node<Msg> {
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
                option![attrs! {At::Value => ""}, "--"],
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

fn data_view(model: &Model) -> Node<Msg> {
    let mut sorted_items = model.items.iter()
        .filter(|r| apply_filters(model, r))
        .collect::<Vec<&Vec<String>>>();
    sorted_items.sort_by(|a, b| sort_items(model, a, b));

    div![
        attrs! {At::Id => "Data", At::Class => "DataView"},
        table![
            thead![tr![model
                .columns
                .iter()
                .enumerate()
                .filter(|(i, _)| model.column_visibility[*i])
                .map(|(_, c)| th![c])
                .collect::<Vec<Node<Msg>>>()]],
            tbody![sorted_items
                .iter()
                .map(|r| tr![r
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| model.column_visibility[*i])
                    .map(|(_, c)| td![c])
                    .collect::<Vec<Node<Msg>>>()])
                .collect::<Vec<Node<Msg>>>()]
        ]
    ]
}

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
                let ordering = item_a[*i]
                    .to_lowercase()
                    .cmp(&item_b[*i].to_lowercase());
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
