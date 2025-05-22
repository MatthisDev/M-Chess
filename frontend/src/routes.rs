use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/download")]
    Download,
    #[at("/info")]
    Info,
    #[at("/create")]
    CreateGame,
    #[at("/game")]
    Game,
    #[not_found]
    #[at("/404")]
    NotFound,
}
