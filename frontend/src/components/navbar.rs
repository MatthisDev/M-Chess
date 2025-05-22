use crate::state::Page;
use yew::prelude::*;

pub fn render_navbar(set_page: Callback<Page>) -> Html {
    html! {
        <nav class="navbar">
            <button onclick={set_page.reform(|_| Page::Home)}>{ "Home" }</button>
            <button onclick={set_page.reform(|_| Page::Info)}>{ "Info" }</button>
            <button onclick={set_page.reform(|_| Page::Install)}>{ "Install" }</button>
        </nav>
    }
}