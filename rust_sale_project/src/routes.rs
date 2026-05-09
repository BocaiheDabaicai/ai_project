use actix_web::web;

use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(handlers::health_check))
            .route("/login", web::post().to(handlers::login))
            .route("/routes", web::get().to(handlers::list_routes))
    );
}
