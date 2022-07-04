mod app;
mod models;
mod services;
mod components;

use app::App;

fn main() {
    yew::start_app::<App>();
}
