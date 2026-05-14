use actix_web::web;
use crate::{handlers, handlers_carton};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(handlers::health_check))
            .route("/login", web::post().to(handlers::login))
            .route("/routes", web::get().to(handlers::list_routes))
            // 纸箱采购
            .route("/carton/requisitions", web::get().to(handlers_carton::list_requisitions))
            .route("/carton/requisitions/{id}", web::get().to(handlers_carton::get_requisition))
            .route("/carton/quotes", web::get().to(handlers_carton::list_quotes))
            .route("/carton/orders", web::get().to(handlers_carton::list_orders))
            .route("/carton/orders/{id}", web::get().to(handlers_carton::get_order))
            .route("/carton/orders/create", web::post().to(handlers_carton::create_order))
            .route("/carton/orders/approve", web::post().to(handlers_carton::approve_order))
            .route("/carton/orders/ship", web::post().to(handlers_carton::ship_order))
            .route("/carton/orders/{id}/shipments", web::get().to(handlers_carton::list_shipments))
            .route("/carton/orders/settle", web::post().to(handlers_carton::settle_order))
    );
}
