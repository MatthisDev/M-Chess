use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq, Debug, Serialize, Deserialize)]
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

impl std::str::FromStr for Route {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Home" => Ok(Route::Home),
            "CreateGame" => Ok(Route::CreateGame),
            "Game" => Ok(Route::Game),
            "Info" => Ok(Route::Info),
            "Download" => Ok(Route::Download),
            _ => Ok(Route::Home),
        }
    }
}
impl ToString for Route {
    fn to_string(&self) -> String {
        match self {
            Route::Home => "Home".to_string(),
            Route::CreateGame => "CreateGame".to_string(),
            Route::Game => "Game".to_string(),
            Route::Info => "Info".to_string(),
            Route::Download => "Download".to_string(),
            _ => "Home".to_string(),
        }
    }
}
