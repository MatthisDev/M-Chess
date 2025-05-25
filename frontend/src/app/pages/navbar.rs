use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <header class="navbar">
            <nav class="navbar-container">
                <Link<Route> to={Route::Home}>
                    <button class="navbar-button">{ "Home" }</button>
                </Link<Route>>
                <Link<Route> to={Route::Info}>
                    <button class="navbar-button">{ "Info" }</button>
                </Link<Route>>
                <Link<Route> to={Route::Download}>
                    <button class="navbar-button">{ "Download" }</button>
                </Link<Route>>
            </nav>
        </header>
    }
}
