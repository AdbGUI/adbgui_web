mod app;
mod models;
mod components;
mod api;

use app::App;

fn main() {
    yew::start_app::<App>();
}
