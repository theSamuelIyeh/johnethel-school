use crate::routes;
use actix_web::web::ServiceConfig;

pub fn init_router(cfg: &mut ServiceConfig) {
    cfg.service(routes::hello_world).service(
        actix_files::Files::new("/static", "templates/static/")
            .show_files_listing()
            .use_last_modified(true),
    );
}
