use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav>
            <ul>
                <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                <li><Link<Route> to={Route::Info}>{ "Info" }</Link<Route>></li>
                <li><Link<Route> to={Route::Download}>{ "Download" }</Link<Route>></li>
            </ul>
        </nav>
    }
}
