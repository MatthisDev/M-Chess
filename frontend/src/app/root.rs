use crate::routes::Route;
use crate::pages::navbar::Navbar;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Root)]
pub fn root() -> Html {
    html! {
        <>
            <Navbar />
            <div class="page-container">
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </div>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::CreateGame => html! { <CreateGame /> },
        Route::Info => html! { <Info /> },
        Route::Download => html! { <Download /> },
    }
}