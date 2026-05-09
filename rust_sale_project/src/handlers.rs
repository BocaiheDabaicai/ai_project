use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Local;

use crate::models::{
    ApiResponse, DbUser, HealthResponse, LoginRequest, LoginResponse, RouteInfo, RoutesResponse,
    UserInfo,
};

#[utoipa::path(
    get,
    path = "/api/health",
    tag = "健康检查",
    responses(
        (status = 200, description = "服务健康状态", body = ApiResponseHealth)
    )
)]
pub async fn health_check() -> HttpResponse {
    let resp = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    HttpResponse::Ok().json(ApiResponse::success(resp))
}

#[utoipa::path(
    post,
    path = "/api/login",
    tag = "用户认证",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "登录成功", body = ApiResponseLogin),
        (status = 401, description = "用户名或密码错误", body = ApiResponseEmpty)
    )
)]
pub async fn login(pool: web::Data<PgPool>, body: web::Json<LoginRequest>) -> HttpResponse {
    let user = sqlx::query_as::<_, DbUser>(
        "SELECT id, username, password_hash, role, avatar FROM users WHERE username = $1",
    )
    .bind(&body.username)
    .fetch_optional(pool.get_ref())
    .await;

    match user {
        Ok(Some(db_user)) => {
            if bcrypt::verify(&body.password, &db_user.password_hash).unwrap_or(false) {
                let token = format!("sale_token_{}", Uuid::new_v4());
                let resp = LoginResponse {
                    user: UserInfo {
                        username: db_user.username,
                        role: db_user.role,
                        avatar: db_user.avatar,
                    },
                    token,
                };
                HttpResponse::Ok().json(ApiResponse::success(resp))
            } else {
                HttpResponse::Unauthorized()
                    .json(ApiResponse::<()>::error(401, "用户名或密码错误"))
            }
        }
        Ok(None) => HttpResponse::Unauthorized()
            .json(ApiResponse::<()>::error(401, "用户名或密码错误")),
        Err(_) => HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::error(500, "数据库查询失败")),
    }
}

#[utoipa::path(
    get,
    path = "/api/routes",
    tag = "开发工具",
    responses(
        (status = 200, description = "所有接口路径列表", body = ApiResponseRoutes)
    )
)]
pub async fn list_routes() -> HttpResponse {
    let routes = vec![
        RouteInfo {
            method: "GET".to_string(),
            path: "/api/health".to_string(),
            description: "健康检查".to_string(),
        },
        RouteInfo {
            method: "POST".to_string(),
            path: "/api/login".to_string(),
            description: "用户登录".to_string(),
        },
        RouteInfo {
            method: "GET".to_string(),
            path: "/api/routes".to_string(),
            description: "接口路径列表".to_string(),
        },
    ];
    let resp = RoutesResponse { routes };
    HttpResponse::Ok().json(ApiResponse::success(resp))
}
