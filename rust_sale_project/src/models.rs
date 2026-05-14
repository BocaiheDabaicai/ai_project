use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

// ===== 通用 =====
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}
impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self { ApiResponse { code: 200, message: "success".to_string(), data: Some(data) } }
    pub fn error(code: i32, message: &str) -> Self { ApiResponse { code, message: message.to_string(), data: None } }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UserInfo { pub username: String, pub role: String, pub avatar: String }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest { pub username: String, pub password: String }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse { pub user: UserInfo, pub token: String }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse { pub status: String, pub version: String, pub timestamp: String }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RouteInfo { pub method: String, pub path: String, pub description: String }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RoutesResponse { pub routes: Vec<RouteInfo> }

#[derive(Debug, FromRow)] #[allow(dead_code)]
pub struct DbUser { pub id: i32, pub username: String, pub password_hash: String, pub role: String, pub avatar: String }

// ===== 纸箱请购单 =====
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CartonRequisition { pub id: i32, pub req_no: String, pub dept: String, pub applicant: String, pub total_qty: i32, pub status: String, pub push_time: String }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CartonRequisitionItem { pub id: i32, pub material_code: String, pub material_name: String, pub spec: String, pub qty: i32, pub unit: String, pub is_new_product: bool, pub oa_price: Option<f64> }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CartonRequisitionDetail { pub req: CartonRequisition, pub items: Vec<CartonRequisitionItem> }

// ===== 纸箱供应商报价 =====
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SupplierQuote { pub id: i32, pub supplier_code: String, pub supplier_name: String, pub material_code: String, pub material_name: String, pub quarter: String, pub price: f64, pub pkg: String }

// ===== 创建采购订单请求 =====
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateOrderRequest {
    pub req_id: i32,
    pub items: Vec<CreateOrderItem>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateOrderItem {
    pub material_code: String,
    pub material_name: String,
    pub spec: String,
    pub total_qty: i32,
    pub unit: String,
    pub strategy: String,
    pub selected_price: f64,
    pub pkg: String,
    pub card_paper: String,
    pub suppliers: Vec<CreateOrderSupplier>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateOrderSupplier {
    pub supplier_code: String,
    pub supplier_name: String,
    pub qty: i32,
    pub unit_price: f64,
    pub test_result: String,
    pub color_diff: String,
    pub quality: String,
    pub score: f64,
    pub is_standard_sample: bool,
    pub has_bulk_history: bool,
    pub production_location: String,
    pub notes: String,
}

// ===== 纸箱采购订单 =====
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CartonOrder {
    pub id: i32, pub order_no: String, pub req_no: String, pub name: String,
    pub total_qty: i32, pub total_delivered: i32, pub total_amount: f64,
    pub status: String, pub approved: bool, pub created_at: String,
}
#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct DbOrderSupplier {
    pub id: i32, pub material_code: String, pub material_name: String,
    pub supplier_code: String, pub supplier_name: String,
    pub qty: i32, pub delivered: i32, pub unit_price: f64,
    pub pkg: String, pub strategy: String, pub card_paper: String,
    pub test_result: String, pub color_diff: String, pub quality: String,
    pub score: f64, pub is_standard_sample: bool, pub has_bulk_history: bool,
    pub production_location: Option<String>, pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CartonOrderSupplier {
    pub id: i32, pub material_code: String, pub material_name: String,
    pub supplier_code: String, pub supplier_name: String,
    pub qty: i32, pub delivered: i32, pub unit_price: f64,
    pub pkg: String, pub strategy: String, pub card_paper: String,
    pub test_result: String, pub color_diff: String, pub quality: String,
    pub score: f64, pub is_standard_sample: bool, pub has_bulk_history: bool,
    pub production_location: String, pub notes: String,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CartonOrderItemGroup {
    pub material_code: String, pub material_name: String, pub spec: String,
    pub total_qty: i32, pub unit: String, pub strategy: String, pub selected_price: f64,
    pub pkg: String, pub card_paper: String,
    pub suppliers: Vec<CartonOrderSupplier>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CartonOrderDetail {
    pub order: CartonOrder,
    pub items: Vec<CartonOrderItemGroup>,
}

// ===== 发货 =====
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ShipRequest {
    pub order_id: i32,
    pub shipments: Vec<ShipItem>,
    pub arrival_time: String,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ShipItem {
    pub material_code: String, pub supplier_code: String,
    pub qty: i32,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CartonShipment {
    pub id: i32, pub batch_no: String, pub material_code: String, pub material_name: String,
    pub supplier_code: String, pub supplier_name: String,
    pub qty: i32, pub unit: String, pub cost: f64,
    pub status: String, pub arrival_time: Option<String>, pub created_at: String,
}

// ===== 审批/结算 =====
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApproveRequest { pub order_id: i32, pub approved_by: String }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SettleRequest { pub order_id: i32, pub supplier_code: String }

// ===== 类型别名 =====
pub type ApiResponseHealth = ApiResponse<HealthResponse>;
pub type ApiResponseLogin = ApiResponse<LoginResponse>;
pub type ApiResponseRoutes = ApiResponse<RoutesResponse>;
pub type ApiResponseEmpty = ApiResponse<()>;
pub type ApiResponseRequisitions = ApiResponse<Vec<CartonRequisition>>;
pub type ApiResponseRequisitionDetail = ApiResponse<CartonRequisitionDetail>;
pub type ApiResponseQuotes = ApiResponse<Vec<SupplierQuote>>;
pub type ApiResponseOrders = ApiResponse<Vec<CartonOrder>>;
pub type ApiResponseOrderDetail = ApiResponse<CartonOrderDetail>;
pub type ApiResponseShipments = ApiResponse<Vec<CartonShipment>>;
