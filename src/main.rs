mod routes;
mod views;

use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let johnethel_school_site = OpenApiService::new(routes::RootRoutes, "Johnethel School", "1.0")
        .server("http://localhost:8080");
    let swagger_ui = johnethel_school_site.swagger_ui();

    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(
            Route::new()
                .nest("/", johnethel_school_site)
                .nest("/test", swagger_ui)
                .nest(
                    "/static",
                    StaticFilesEndpoint::new("static").show_files_listing(),
                ),
        )
        .await
}
