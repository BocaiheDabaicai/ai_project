use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod handlers;
mod routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health_check,
        handlers::login,
    ),
    components(
        schemas(
            models::ApiResponseHealth,
            models::ApiResponseLogin,
            models::ApiResponseEmpty,
            models::HealthResponse,
            models::LoginRequest,
            models::LoginResponse,
            models::UserInfo,
        )
    ),
    tags(
        (name = "健康检查", description = "服务状态检查接口"),
        (name = "用户认证", description = "用户登录相关接口"),
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
    let openapi = ApiDoc::openapi();

    println!("=== Rust Sale Backend Server ===");
    println!("Server running at http://0.0.0.0:8080");
    println!("API docs:     http://localhost:8080/swagger-ui/");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .configure(routes::configure)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
