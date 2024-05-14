use html_to_string_macro::html;

use crate::views::components;

pub fn about() -> String {
    html! {
        <div class="h-screen flex flex-col" id="about-page">
        {components::navbar("about".to_owned())}
        </div>
    }
}
