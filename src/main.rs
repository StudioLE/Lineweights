mod app;
mod chart;
mod entries;
mod import;
mod nav;
mod prelude;
mod table;

use crate::app::App;
use dioxus::prelude::launch;

fn main() {
    launch(App);
}
