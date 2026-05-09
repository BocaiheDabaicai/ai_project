use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod db;
mod handlers;
mod models;
mod routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health_check,
        handlers::login,
        handlers::list_routes,
    ),
    components(
        schemas(
            models::ApiResponseHealth,
            models::ApiResponseLogin,
            models::ApiResponseRoutes,
            models::ApiResponseEmpty,
            models::HealthResponse,
            models::LoginRequest,
            models::LoginResponse,
            models::RouteInfo,
            models::RoutesResponse,
            models::UserInfo,
        )
    ),
    tags(
        (name = "健康检查", description = "服务状态检查接口"),
        (name = "用户认证", description = "用户登录相关接口"),
        (name = "开发工具", description = "开发调试辅助接口"),
    ),
    info(
        title = "采购运营平台 API",
        version = "0.1.0",
        description = "Rust后端服务 - 为采购运营平台提供数据接口"
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let pool = db::create_pool().await;
    db::run_migrations(&pool).await;
    db::seed_admin(&pool).await;

    let openapi = ApiDoc::openapi();

    println!();
    println!("=== Rust Sale Backend Server ===");
    println!("Server running at http://0.0.0.0:8080");
    println!("API docs:     http://localhost:8080/swagger-ui/");
    println!("Database:     PostgreSQL @ localhost:5432/sale_db");
    println!();
    println!("Available API routes:");
    println!("  GET    /api/health   - 健康检查");
    println!("  POST   /api/login    - 用户登录");
    println!("  GET    /api/routes   - 接口路径列表");
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
