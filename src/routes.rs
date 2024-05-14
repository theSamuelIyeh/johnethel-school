use crate::views::{about, blog, contact, forgot_password, gallery, index, layout, login};
use poem_openapi::{payload::Html, OpenApi};

pub struct RootRoutes;

#[OpenApi]
impl RootRoutes {
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> Html<String> {
        Html(layout::layout(
            String::from("Johnethel School | Home"),
            index::index(),
        ))
    }

    #[oai(path = "/about", method = "get")]
    async fn about(&self) -> Html<String> {
        Html(layout::layout(
            String::from("Johnethel School | About Us"),
            about::about(),
        ))
    }

    #[oai(path = "/login", method = "get")]
    async fn login(&self) -> Html<String> {
        Html(layout::layout(
            String::from("Johnethel School | Login"),
            login::login(),
        ))
    }

    #[oai(path = "/forgot-password", method = "get")]
    async fn forgot_password(&self) -> Html<String> {
        Html(layout::layout(
            String::from("Johnethel School | Forgot Password"),
            forgot_password::forgot_password(),
        ))
    }

    #[oai(path = "/contact", method = "get")]
    async fn contact(&self) -> Html<String> {
        Html(layout::layout(
            String::from("Johnethel School | Contact Us"),
            contact::contact(),
        ))
    }

    #[oai(path = "/gallery", method = "get")]
    async fn gallery(&self) -> Html<String> {
        Html(layout::layout(
            String::from("Johnethel School | Gallery"),
            gallery::gallery(),
        ))
    }

    #[oai(path = "/blog", method = "get")]
    async fn blog(&self) -> Html<String> {
        Html(layout::layout(
            String::from("Johnethel School | Blog"),
            blog::blog(),
        ))
    }
}
