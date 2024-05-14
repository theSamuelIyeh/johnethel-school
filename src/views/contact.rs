use super::components;
use html_to_string_macro::html;

pub fn contact() -> String {
    html! {
        <div class="h-screen flex flex-col" id="about-page">
        {components::navbar("contact".to_owned())}
        <h1>"this is contact"</h1>
        </div>
    }
}
