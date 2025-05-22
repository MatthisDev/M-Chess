use crate::handlers::change_theme;
use yew::prelude::*;

pub fn render_palette() -> Html {
    html! {
        <div class="palette">
            <button onclick={change_theme("default".to_string())}>{ "Default Theme" }</button>
            <button onclick={change_theme("brown".to_string())}>{ "Brown Theme" }</button>
            <button onclick={change_theme("blue".to_string())}>{ "Blue Theme" }</button>
        </div>
    }
}