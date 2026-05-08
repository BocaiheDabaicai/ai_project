use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::Local;
use utoipa;

use crate::models::{
    ApiResponse, HealthResponse, LoginRequest, LoginResponse, UserInfo,
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
    )
)]
pub async fn login(body: web::Json<LoginRequest>) -> HttpResponse {
    if body.username == "admin" && body.password == "123456" {
        let user_info = UserInfo {
            username: body.username.clone(),
            role: "管理员".to_string(),
            avatar: "".to_string(),
        };
        let token = format!("sale_token_{}", Uuid::new_v4());
        let resp = LoginResponse {
            user: user_info,
            token,
        };
        HttpResponse::Ok().json(ApiResponse::success(resp))
    } else {
        HttpResponse::Ok().json(ApiResponse::<()>::error(401, "用户名或密码错误"))
    }
}
