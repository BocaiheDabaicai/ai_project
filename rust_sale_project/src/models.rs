use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, message: &str) -> Self {
        ApiResponse {
            code,
            message: message.to_string(),
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UserInfo {
    pub username: String,
    pub role: String,
    pub avatar: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub user: UserInfo,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RouteInfo {
    pub method: String,
    pub path: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RoutesResponse {
    pub routes: Vec<RouteInfo>,
}

#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct DbUser {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub avatar: String,
}

pub type ApiResponseHealth = ApiResponse<HealthResponse>;
pub type ApiResponseLogin = ApiResponse<LoginResponse>;
pub type ApiResponseRoutes = ApiResponse<RoutesResponse>;
pub type ApiResponseEmpty = ApiResponse<()>;
