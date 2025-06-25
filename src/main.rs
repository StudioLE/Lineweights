use crate::components::App;
use dioxus::prelude::*;
mod components;
mod schema;
mod state;

fn main() {
    launch(App);
}
