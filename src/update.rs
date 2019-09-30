use csv::Reader;
use futures::Future;
use seed::fetch::FetchObject;
use seed::prelude::*;
use seed::Request;

use crate::model::Model;

#[derive(Clone)]
pub enum Msg {
    ChangeFilterColumn(usize, String),
    ChangeFilterValue(usize, String),
    ChangeColumnVisibility(usize, String),
    ChangeSortColumn(usize, String),
    DataFetched(FetchObject<String>),
    FetchData,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangeFilterColumn(index, new_value) => {
            handle_change_filter_column(model, index, new_value)
        }
        Msg::ChangeFilterValue(index, new_value) => {
            handle_change_filter_value(model, index, new_value)
        }
        Msg::ChangeColumnVisibility(index, _) => handle_change_column_visibility(model, index),
        Msg::ChangeSortColumn(index, new_value) => {
            handle_change_sort_column(model, index, new_value)
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
        },
    }
}

fn handle_change_filter_column(model: &mut Model, index: usize, new_column: String) {
    model.filters[index].0 = new_column.parse::<usize>().ok();
}

fn handle_change_filter_value(model: &mut Model, index: usize, new_value: String) {
    model.filters[index].1 = new_value;
}

fn handle_change_column_visibility(model: &mut Model, index: usize) {
    model.column_visibility[index] = !model.column_visibility[index];
}

fn handle_change_sort_column(model: &mut Model, index: usize, new_column: String) {
    model.sort_columns[index] = new_column.parse::<usize>().ok()
}

fn fetch_data() -> impl Future<Item=Msg, Error=Msg> {
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

    model.filters = vec![(Option::None, String::new()); 3];
    model.column_visibility = vec![true; model.columns.len()];
    model.sort_columns = vec![Option::None; 3];
}
