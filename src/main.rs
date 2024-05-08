mod router;
mod routes;
mod templates;
mod utils;

use actix_web::{HttpServer, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(router::init_router)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
