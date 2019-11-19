use csv::Reader;
use futures::Future;
use seed::fetch::FetchObject;
use seed::prelude::*;
use seed::Request;

use crate::model::{Model, ColumnVisibility, Filter, BooleanOp};

#[derive(Clone)]
pub enum Msg {
    AddFilter,
    RemoveFilter(usize),
    ChangeFilterBooleanOp(usize, String),
    ChangeFilterColumn(usize, String),
    ChangeFilterValue(usize, String),
    ChangeColumnVisibility(usize, String),
    ChangeAllColumnVisibility(String),
    AddSortColumn,
    RemoveSortColumn(usize),
    ChangeSortColumn(usize, String),
    DataFetched(FetchObject<String>),
    FetchData,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::AddFilter => { model.filters.push(Filter::default());}
        Msg::RemoveFilter(index) => {
            model.filters.remove(index);
            if index == 0 && !model.filters.is_empty() {
                model.filters[0].boolean_op = match model.filters[0].boolean_op {
                    BooleanOp::Or => BooleanOp::And,
                    BooleanOp::OrNot => BooleanOp::AndNot,
                    op => op
                }
            }
        }
        Msg::ChangeFilterBooleanOp(index, new_op_string) => {
            model.filters[index].boolean_op = match new_op_string.as_ref() {
                "And" => BooleanOp::And,
                "AndNot" => BooleanOp::AndNot,
                "Or" => BooleanOp::Or,
                "OrNot" => BooleanOp::OrNot,
                _ => panic!("Unexpected BooleanOp")
            }
        }
        Msg::ChangeFilterColumn(index, new_value) => {
            model.filters[index].column = new_value.parse::<usize>().ok();
        }
        Msg::ChangeFilterValue(index, new_value) => {
            model.filters[index].value = new_value;
        }
        Msg::ChangeColumnVisibility(index, new_value) => {
            model.column_visibility[index] = match new_value.as_ref() {
                "Auto" => ColumnVisibility::Auto,
                "Hidden" => ColumnVisibility::Hidden,
                "Shown" => ColumnVisibility::Shown,
                _ => panic!("Unexpected ColumnVisibility")
            }
        },
        Msg::ChangeAllColumnVisibility(new_value) => {
            if !new_value.is_empty() {
                let new_value = match new_value.as_ref() {
                    "Auto" => ColumnVisibility::Auto,
                    "Hidden" => ColumnVisibility::Hidden,
                    "Shown" => ColumnVisibility::Shown,
                    _ => panic!("Unexpected ColumnVisibility")
                };
                for index in 0..model.column_visibility.len() {
                    model.column_visibility[index] = new_value;
                }
            }
        }
        Msg::AddSortColumn => { model.sort_columns.push(Option::None); }
        Msg::RemoveSortColumn(index) => { model.sort_columns.remove(index); }
        Msg::ChangeSortColumn(index, new_value) => {
            model.sort_columns[index] = new_value.parse::<usize>().ok()
        }
        Msg::FetchData => {
            orders.skip().perform_cmd(fetch_data());
        }
        Msg::DataFetched(fetch_object) => match fetch_object.response() {
            Ok(response) => {
                load_dataset(model, response.data);
            }
            Err(fail_reason) => {
                error!(format!("Fetch error: {:#?}", fail_reason));
                orders.skip();
            }
        }
    }
}

fn fetch_data() -> impl Future<Item = Msg, Error = Msg> {
    Request::new("/public/acadian-dress.csv").fetch_string(Msg::DataFetched)
}

fn load_dataset(model: &mut Model, data: String) {
    model.items = Vec::new();

    let mut reader = Reader::from_reader(data.as_bytes());
    model.columns = reader
        .headers()
        .unwrap()
        .iter()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    for row_result in reader.records() {
        let row = row_result.unwrap();
        model
            .items
            .push(row.iter().map(|s| s.to_owned()).collect::<Vec<String>>());
    }

    model.column_visibility = vec![ColumnVisibility::Auto; model.columns.len()];
}
