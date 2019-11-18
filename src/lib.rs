#[macro_use]
extern crate seed;

use seed::prelude::*;

use model::Model;
use update::Msg;

mod model;
mod update;
mod view;

#[wasm_bindgen(start)]
pub fn render() {
    let app = seed::App::build(|_, _| Init::new(Model::default()), update::update, view::view)
        .build_and_start();

    app.update(Msg::FetchData);
}