mod app;
mod chart;
mod entries;
mod extensions;
mod import;
mod nav;
mod prelude;
mod state;
mod statistics;
mod table;
#[allow(dead_code)]
mod verify;

use crate::app::App;
use dioxus::prelude::launch;

fn main() {
    launch(App);
}
