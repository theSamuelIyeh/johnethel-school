use super::components;
use html_to_string_macro::html;

pub fn index() -> String {
    html! {
        <div class="flex flex-col h-screen" id="index-page">
    {components::navbar("home".to_owned())}
    {components::hero()}
    </div>
        }
}
