use frontend::root::Root;
use yew::prelude::*;

fn main() {
    console_log::init_with_level(log::Level::Debug).expect("log init failed");
    log::info!("Starting app...");

    // Import navbar.css
    let navbar_css = gloo_utils::document()
        .create_element("link")
        .unwrap();
    navbar_css
        .set_attribute("rel", "stylesheet")
        .unwrap();
    navbar_css
        .set_attribute("href", "/static/styles/navbar.css")
        .unwrap();
    gloo_utils::document().head().unwrap().append_child(&navbar_css).unwrap();

    // Import home.css
    let home_css = gloo_utils::document()
        .create_element("link")
        .unwrap();
    home_css
        .set_attribute("rel", "stylesheet")
        .unwrap();
    home_css
        .set_attribute("href", "/static/styles/home.css")
        .unwrap();
    gloo_utils::document().head().unwrap().append_child(&home_css).unwrap();

    // Import create_game.css
    let create_game_css = gloo_utils::document()
        .create_element("link")
        .unwrap();
    create_game_css
        .set_attribute("rel", "stylesheet")
        .unwrap();
    create_game_css
        .set_attribute("href", "/static/styles/create_game.css")
        .unwrap();
    gloo_utils::document().head().unwrap().append_child(&create_game_css).unwrap();

    // Import game.css
    let game_css = gloo_utils::document()
        .create_element("link")
        .unwrap();
    game_css
        .set_attribute("rel", "stylesheet")
        .unwrap();
    game_css
        .set_attribute("href", "/static/styles/game.css")
        .unwrap();
    gloo_utils::document().head().unwrap().append_child(&game_css).unwrap();

    // Import download.css
    let download_css = gloo_utils::document()
        .create_element("link")
        .unwrap();
    download_css
        .set_attribute("rel", "stylesheet")
        .unwrap();
    download_css
        .set_attribute("href", "/static/styles/download.css")
        .unwrap();
    gloo_utils::document().head().unwrap().append_child(&download_css).unwrap();

    yew::Renderer::<Root>::new().render();
}
