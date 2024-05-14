use super::components;
use html_to_string_macro::html;

pub fn blog() -> String {
    html! {
        <div class="h-screen flex flex-col" id="about-page">
        {components::navbar("blog".to_owned())}
        <h1>"this is contact"</h1>
        </div>
    }
}
