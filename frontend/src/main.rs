mod components;
mod state;
mod handlers;
mod messages;
mod sharedenums;

use components::app::App;
use yew::Renderer;

fn main() {
    yew::Renderer::<App>::new().render();
}