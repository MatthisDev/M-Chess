use yew::prelude::*;

#[function_component(NotFound)]
pub fn notfound() -> Html {
    html! {
        <div>
            <h1>{ "404 NOT FOUND" }</h1>
        </div>
    }
}
