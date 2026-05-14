use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod db;
mod handlers;
mod handlers_carton;
mod models;
mod routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health_check,
        handlers::login,
        handlers::list_routes,
        handlers_carton::list_requisitions,
        handlers_carton::get_requisition,
        handlers_carton::list_quotes,
        handlers_carton::list_orders,
        handlers_carton::get_order,
        handlers_carton::create_order,
        handlers_carton::approve_order,
        handlers_carton::ship_order,
        handlers_carton::list_shipments,
        handlers_carton::settle_order,
    ),
    components(
        schemas(
            models::ApiResponseHealth,
            models::ApiResponseLogin,
            models::ApiResponseRoutes,
            models::ApiResponseEmpty,
            models::ApiResponseRequisitions,
            models::ApiResponseRequisitionDetail,
            models::ApiResponseQuotes,
            models::ApiResponseOrders,
            models::ApiResponseOrderDetail,
            models::ApiResponseShipments,
            models::HealthResponse,
            models::LoginRequest,
            models::LoginResponse,
            models::RouteInfo,
            models::RoutesResponse,
            models::UserInfo,
            models::CartonRequisition,
            models::CartonRequisitionItem,
            models::CartonRequisitionDetail,
            models::SupplierQuote,
            models::CreateOrderRequest,
            models::CreateOrderItem,
            models::CreateOrderSupplier,
            models::CartonOrder,
            models::CartonOrderSupplier,
            models::CartonOrderItemGroup,
            models::CartonOrderDetail,
            models::ShipRequest,
            models::ShipItem,
            models::CartonShipment,
            models::ApproveRequest,
            models::SettleRequest,
        )
    ),
    tags(
        (name = "健康检查", description = "服务状态检查接口"),
        (name = "用户认证", description = "用户登录相关接口"),
        (name = "开发工具", description = "开发调试辅助接口"),
        (name = "纸箱采购-请购单", description = "纸箱请购单管理"),
        (name = "纸箱采购-报价", description = "供应商季度报价"),
        (name = "纸箱采购-订单", description = "纸箱采购订单管理"),
        (name = "纸箱采购-发货结算", description = "发货批次与结算管理"),
    ),
    info(
        title = "采购运营平台 API",
        version = "0.2.0",
        description = "Rust后端服务 - 为采购运营平台提供数据接口（含纸箱采购模块）"
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let pool = db::create_pool().await;
    db::run_migrations(&pool).await;
    db::seed_admin(&pool).await;
    db::seed_carton_data(&pool).await;

    let openapi = ApiDoc::openapi();

    println!();
    println!("=== Rust Sale Backend Server ===");
    println!("Server running at http://0.0.0.0:8080");
    println!("API docs:     http://localhost:8080/swagger-ui/");
    println!("Database:     {}", std::env::var("DATABASE_URL").unwrap_or_default());
    println!();
    println!("Available API routes:");
    println!("  GET    /api/health                     - 健康检查");
    println!("  POST   /api/login                      - 用户登录");
    println!("  GET    /api/routes                     - 接口路径列表");
    println!("  GET    /api/carton/requisitions        - 请购单列表");
    println!("  GET    /api/carton/requisitions/:id    - 请购单详情");
    println!("  GET    /api/carton/quotes              - 供应商报价");
    println!("  GET    /api/carton/orders              - 采购订单列表");
    println!("  GET    /api/carton/orders/:id          - 采购订单详情");
    println!("  POST   /api/carton/orders/create       - 创建采购订单");
    println!("  POST   /api/carton/orders/approve      - 审批订单");
    println!("  POST   /api/carton/orders/ship         - 发货");
    println!("  GET    /api/carton/orders/:id/shipments- 批次列表");
    println!("  POST   /api/carton/orders/settle       - 结算");
    println!();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
