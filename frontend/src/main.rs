use frontend::root::Root;
use yew::prelude::*;

fn main() {
    console_log::init_with_level(log::Level::Debug).expect("log init failed");
    log::info!("Starting app...");

    yew::Renderer::<Root>::new().render();
}
