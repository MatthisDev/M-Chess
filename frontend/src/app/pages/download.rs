use yew::prelude::*;

#[function_component(Download)]
pub fn home() -> Html {
    html! {
        <div class="install-page">
            <h2>{ "Installation" }</h2>
            <a href="static/M-Chess_Project.zip" download="M-Chess_Project.zip">{ "Download the project" }</a><br />
            <a href="static/M-Chess_Report.pdf" download="M-Chess_Report.pdf">{ "Download the report (PDF)" }</a>
        </div>
    }
}
