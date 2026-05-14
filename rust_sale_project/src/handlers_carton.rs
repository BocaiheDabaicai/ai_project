use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::models::*;

// ===== 请购单列表 =====
#[utoipa::path(get, path = "/api/carton/requisitions", tag = "纸箱采购-请购单",
    responses((status = 200, body = ApiResponseRequisitions)))]
pub async fn list_requisitions(pool: web::Data<PgPool>) -> HttpResponse {
    let rows = sqlx::query_as::<_, (i32, String, String, String, i32, String, String)>(
        "SELECT id, req_no, dept, applicant, total_qty, status, to_char(push_time, 'YYYY-MM-DD HH24:MI:SS') FROM carton_requisitions ORDER BY id DESC"
    ).fetch_all(pool.get_ref()).await;
    match rows {
        Ok(r) => {
            let list: Vec<CartonRequisition> = r.into_iter().map(|(id, req_no, dept, applicant, total_qty, status, push_time)| {
                CartonRequisition { id, req_no, dept, applicant, total_qty, status, push_time }
            }).collect();
            HttpResponse::Ok().json(ApiResponse::success(list))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string())),
    }
}

// ===== 请购单详情 =====
#[utoipa::path(get, path = "/api/carton/requisitions/{id}", tag = "纸箱采购-请购单",
    responses((status = 200, body = ApiResponseRequisitionDetail)))]
pub async fn get_requisition(pool: web::Data<PgPool>, path: web::Path<i32>) -> HttpResponse {
    let req_id = path.into_inner();
    let req_row = sqlx::query_as::<_, (i32, String, String, String, i32, String, String)>(
        "SELECT id, req_no, dept, applicant, total_qty, status, to_char(push_time, 'YYYY-MM-DD HH24:MI:SS') FROM carton_requisitions WHERE id = $1"
    ).bind(req_id).fetch_optional(pool.get_ref()).await;
    match req_row {
        Ok(Some((id, req_no, dept, applicant, total_qty, status, push_time))) => {
            let req = CartonRequisition { id, req_no, dept, applicant, total_qty, status, push_time };
            let item_rows = sqlx::query_as::<_, (i32, String, String, String, i32, String, bool, Option<f64>)>(
                "SELECT id, material_code, material_name, spec, qty, unit, is_new_product, oa_price::float8 FROM carton_requisition_items WHERE req_id = $1"
            ).bind(req_id).fetch_all(pool.get_ref()).await.unwrap_or_default();
            let items = item_rows.into_iter().map(|(id, material_code, material_name, spec, qty, unit, is_new_product, oa_price)| {
                CartonRequisitionItem { id, material_code, material_name, spec, qty, unit, is_new_product, oa_price }
            }).collect();
            HttpResponse::Ok().json(ApiResponse::success(CartonRequisitionDetail { req, items }))
        }
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error(404, "请购单不存在")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string())),
    }
}

// ===== 供应商报价 =====
#[utoipa::path(get, path = "/api/carton/quotes", tag = "纸箱采购-报价",
    responses((status = 200, body = ApiResponseQuotes)))]
pub async fn list_quotes(pool: web::Data<PgPool>) -> HttpResponse {
    let rows = sqlx::query_as::<_, (i32, String, String, String, String, String, f64, String)>(
        "SELECT id, supplier_code, supplier_name, material_code, material_name, quarter, price::float8, pkg FROM carton_supplier_quotes ORDER BY material_code, price"
    ).fetch_all(pool.get_ref()).await;
    match rows {
        Ok(r) => {
            let list: Vec<SupplierQuote> = r.into_iter().map(|(id, supplier_code, supplier_name, material_code, material_name, quarter, price, pkg)| {
                SupplierQuote { id, supplier_code, supplier_name, material_code, material_name, quarter, price, pkg }
            }).collect();
            HttpResponse::Ok().json(ApiResponse::success(list))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string())),
    }
}

// ===== 创建采购订单 =====
#[utoipa::path(post, path = "/api/carton/orders/create", tag = "纸箱采购-订单",
    request_body = CreateOrderRequest, responses((status = 200, body = ApiResponseEmpty)))]
pub async fn create_order(pool: web::Data<PgPool>, body: web::Json<CreateOrderRequest>) -> HttpResponse {
    let req_row = sqlx::query_as::<_, (String,)>("SELECT req_no FROM carton_requisitions WHERE id = $1")
        .bind(body.req_id).fetch_optional(pool.get_ref()).await;
    let _req_no = match req_row {
        Ok(Some((rn,))) => rn,
        _ => return HttpResponse::NotFound().json(ApiResponse::<()>::error(404, "请购单不存在")),
    };
    let order_no = format!("PO{}", chrono::Local::now().format("%Y%m%d%H%M%S"));
    let total_qty: i32 = body.items.iter().map(|i| i.total_qty).sum();
    let total_amount: f64 = body.items.iter().map(|i| i.selected_price * i.total_qty as f64).sum();
    let result = sqlx::query(
        "INSERT INTO carton_orders (order_no, req_id, req_no, name, total_qty, total_amount, status) VALUES ($1,$2,$3,$4,$5,$6,'待审批')"
    ).bind(&order_no).bind(body.req_id).bind(&_req_no).bind("纸箱批量采购").bind(total_qty).bind(total_amount)
        .execute(pool.get_ref()).await;
    let order_id: i32 = match result {
        Ok(_) => {
            let id_row: (i32,) = sqlx::query_as("SELECT id FROM carton_orders WHERE order_no = $1")
                .bind(&order_no).fetch_one(pool.get_ref()).await.unwrap();
            id_row.0
        }
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string())),
    };
    for item in &body.items {
        for s in &item.suppliers {
            let _ = sqlx::query(
                "INSERT INTO carton_order_suppliers (order_id, material_code, material_name, supplier_code, supplier_name, qty, unit_price, pkg, strategy, card_paper, test_result, color_diff, quality, score, is_standard_sample, has_bulk_history, production_location, notes) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18)"
            ).bind(order_id).bind(&item.material_code).bind(&item.material_name)
                .bind(&s.supplier_code).bind(&s.supplier_name).bind(s.qty).bind(s.unit_price)
                .bind(&item.pkg).bind(&item.strategy).bind(&item.card_paper)
                .bind(&s.test_result).bind(&s.color_diff).bind(&s.quality)
                .bind(s.score).bind(s.is_standard_sample).bind(s.has_bulk_history)
                .bind(&s.production_location).bind(&s.notes)
                .execute(pool.get_ref()).await;
        }
    }
    let _ = sqlx::query("UPDATE carton_requisitions SET status = '已处理' WHERE id = $1")
        .bind(body.req_id).execute(pool.get_ref()).await;
    HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"order_no": order_no, "order_id": order_id})))
}

// ===== 领导审批 =====
#[utoipa::path(post, path = "/api/carton/orders/approve", tag = "纸箱采购-订单",
    request_body = ApproveRequest, responses((status = 200, body = ApiResponseEmpty)))]
pub async fn approve_order(pool: web::Data<PgPool>, body: web::Json<ApproveRequest>) -> HttpResponse {
    let result = sqlx::query("UPDATE carton_orders SET approved = true, approved_by = $1, approved_at = NOW(), status = '已下单' WHERE id = $2 AND approved = false")
        .bind(&body.approved_by).bind(body.order_id).execute(pool.get_ref()).await;
    match result {
        Ok(r) if r.rows_affected() > 0 => {
            HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"message": "审批通过，已通知供应商"})))
        }
        Ok(_) => HttpResponse::NotFound().json(ApiResponse::<()>::error(404, "订单不存在或已审批")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string())),
    }
}

// ===== 订单列表 =====
#[utoipa::path(get, path = "/api/carton/orders", tag = "纸箱采购-订单",
    responses((status = 200, body = ApiResponseOrders)))]
pub async fn list_orders(pool: web::Data<PgPool>) -> HttpResponse {
    let rows = sqlx::query_as::<_, (i32, String, String, String, i32, i32, f64, String, bool, String)>(
        "SELECT id, order_no, COALESCE(req_no,''), COALESCE(name,''), total_qty, total_delivered, total_amount::float8, status, approved, to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') FROM carton_orders ORDER BY id DESC"
    ).fetch_all(pool.get_ref()).await;
    match rows {
        Ok(r) => {
            let list: Vec<CartonOrder> = r.into_iter().map(|(id,order_no,req_no,name,total_qty,total_delivered,total_amount,status,approved,created_at)| {
                CartonOrder { id, order_no, req_no, name, total_qty, total_delivered, total_amount, status, approved, created_at }
            }).collect();
            HttpResponse::Ok().json(ApiResponse::success(list))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string())),
    }
}

// ===== 订单详情 =====
#[utoipa::path(get, path = "/api/carton/orders/{id}", tag = "纸箱采购-订单",
    responses((status = 200, body = ApiResponseOrderDetail)))]
pub async fn get_order(pool: web::Data<PgPool>, path: web::Path<i32>) -> HttpResponse {
    let ord_id = path.into_inner();
    let ord_row = sqlx::query_as::<_, (i32, String, String, String, i32, i32, f64, String, bool, String)>(
        "SELECT id, order_no, COALESCE(req_no,''), COALESCE(name,''), total_qty, total_delivered, total_amount, status, approved, to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') FROM carton_orders WHERE id = $1"
    ).bind(ord_id).fetch_optional(pool.get_ref()).await;
    match ord_row {
        Ok(Some((id,order_no,req_no,name,total_qty,total_delivered,total_amount,status,approved,created_at))) => {
            let order = CartonOrder { id, order_no, req_no, name, total_qty, total_delivered, total_amount, status, approved, created_at };
            let s_rows = sqlx::query_as::<_, DbOrderSupplier>(
                "SELECT id,material_code,material_name,supplier_code,supplier_name,qty,delivered,unit_price::float8,pkg,strategy,card_paper,test_result,color_diff,quality,score::float8,is_standard_sample,has_bulk_history,production_location,notes FROM carton_order_suppliers WHERE order_id = $1"
            ).bind(ord_id).fetch_all(pool.get_ref()).await.unwrap_or_default();
            let mut groups: Vec<CartonOrderItemGroup> = vec![];
            let mut seen: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
            for db_s in &s_rows {
                let sup = CartonOrderSupplier {
                    id: db_s.id, material_code: db_s.material_code.clone(), material_name: db_s.material_name.clone(),
                    supplier_code: db_s.supplier_code.clone(), supplier_name: db_s.supplier_name.clone(),
                    qty: db_s.qty, delivered: db_s.delivered, unit_price: db_s.unit_price,
                    pkg: db_s.pkg.clone(), strategy: db_s.strategy.clone(), card_paper: db_s.card_paper.clone(),
                    test_result: db_s.test_result.clone(), color_diff: db_s.color_diff.clone(), quality: db_s.quality.clone(),
                    score: db_s.score, is_standard_sample: db_s.is_standard_sample, has_bulk_history: db_s.has_bulk_history,
                    production_location: db_s.production_location.clone().unwrap_or_default(),
                    notes: db_s.notes.clone().unwrap_or_default(),
                };
                if let Some(&idx) = seen.get(&db_s.material_code) {
                    groups[idx].suppliers.push(sup);
                    groups[idx].total_qty += db_s.qty;
                } else {
                    seen.insert(db_s.material_code.clone(), groups.len());
                    groups.push(CartonOrderItemGroup {
                        material_code: db_s.material_code.clone(), material_name: db_s.material_name.clone(),
                        spec: String::new(), total_qty: db_s.qty, unit: "个".into(),
                        strategy: db_s.strategy.clone(), selected_price: db_s.unit_price,
                        pkg: db_s.pkg.clone(), card_paper: db_s.card_paper.clone(),
                        suppliers: vec![sup],
                    });
                }
            }
            HttpResponse::Ok().json(ApiResponse::success(CartonOrderDetail { order, items: groups }))
        }
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error(404, "订单不存在")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string())),
    }
}

// ===== 发货 =====
#[utoipa::path(post, path = "/api/carton/orders/ship", tag = "纸箱采购-发货结算",
    request_body = ShipRequest, responses((status = 200, body = ApiResponseEmpty)))]
pub async fn ship_order(pool: web::Data<PgPool>, body: web::Json<ShipRequest>) -> HttpResponse {
    let batch_no = format!("BATCH-{}", chrono::Local::now().format("%Y%m%d%H%M%S"));
    let mut total_shipped = 0;
    for s in &body.shipments {
        let cost = {
            let price_row = sqlx::query_as::<_, (f64,)>("SELECT unit_price FROM carton_order_suppliers WHERE order_id = $1 AND material_code = $2 AND supplier_code = $3")
                .bind(body.order_id).bind(&s.material_code).bind(&s.supplier_code)
                .fetch_optional(pool.get_ref()).await;
            match price_row { Ok(Some((p,))) => p * s.qty as f64, _ => 0.0 }
        };
        let material_name = {
            let row = sqlx::query_as::<_, (String,)>("SELECT material_name FROM carton_order_suppliers WHERE order_id = $1 AND material_code = $2 AND supplier_code = $3 LIMIT 1")
                .bind(body.order_id).bind(&s.material_code).bind(&s.supplier_code).fetch_optional(pool.get_ref()).await;
            match row { Ok(Some((n,))) => n, _ => String::new() }
        };
        let supplier_name = {
            let row = sqlx::query_as::<_, (String,)>("SELECT supplier_name FROM carton_order_suppliers WHERE order_id = $1 AND supplier_code = $2 LIMIT 1")
                .bind(body.order_id).bind(&s.supplier_code).fetch_optional(pool.get_ref()).await;
            match row { Ok(Some((n,))) => n, _ => String::new() }
        };
        let _ = sqlx::query(
            "INSERT INTO carton_shipments (order_id, batch_no, material_code, material_name, supplier_code, supplier_name, qty, unit, cost, status, arrival_time) VALUES ($1,$2,$3,$4,$5,$6,$7,'个',$8,'已下单',$9)"
        ).bind(body.order_id).bind(&batch_no).bind(&s.material_code).bind(&material_name)
            .bind(&s.supplier_code).bind(&supplier_name).bind(s.qty).bind(cost)
            .bind(&body.arrival_time).execute(pool.get_ref()).await;
        let _ = sqlx::query("UPDATE carton_order_suppliers SET delivered = delivered + $1 WHERE order_id = $2 AND material_code = $3 AND supplier_code = $4")
            .bind(s.qty).bind(body.order_id).bind(&s.material_code).bind(&s.supplier_code).execute(pool.get_ref()).await;
        total_shipped += s.qty;
    }
    let _ = sqlx::query("UPDATE carton_orders SET total_delivered = total_delivered + $1, status = CASE WHEN total_delivered + $1 >= total_qty THEN '已发货' ELSE status END WHERE id = $2")
        .bind(total_shipped).bind(body.order_id).execute(pool.get_ref()).await;
    HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"batch_no": batch_no, "message": "发货成功，已通知供应商"})))
}

// ===== 批次列表 =====
#[utoipa::path(get, path = "/api/carton/orders/{id}/shipments", tag = "纸箱采购-发货结算",
    responses((status = 200, body = ApiResponseShipments)))]
pub async fn list_shipments(pool: web::Data<PgPool>, path: web::Path<i32>) -> HttpResponse {
    let rows = sqlx::query_as::<_, (i32,String,String,String,String,String,i32,String,f64,String,Option<String>,String)>(
        "SELECT id, batch_no, material_code, material_name, supplier_code, supplier_name, qty, unit, cost::float8, status, to_char(arrival_time, 'YYYY-MM-DD HH24:MI:SS'), to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') FROM carton_shipments WHERE order_id = $1 ORDER BY id DESC"
    ).bind(path.into_inner()).fetch_all(pool.get_ref()).await;
    match rows {
        Ok(r) => {
            let list: Vec<CartonShipment> = r.into_iter().map(|(id,batch_no,mc,mn,sc,sn,qty,unit,cost,status,arrival_time,created_at)| {
                CartonShipment { id, batch_no, material_code: mc, material_name: mn, supplier_code: sc, supplier_name: sn, qty, unit, cost, status, arrival_time, created_at }
            }).collect();
            HttpResponse::Ok().json(ApiResponse::success(list))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string())),
    }
}

// ===== 结算 =====
#[utoipa::path(post, path = "/api/carton/orders/settle", tag = "纸箱采购-发货结算",
    request_body = SettleRequest, responses((status = 200, body = ApiResponseEmpty)))]
pub async fn settle_order(pool: web::Data<PgPool>, body: web::Json<SettleRequest>) -> HttpResponse {
    let _ = sqlx::query("UPDATE carton_shipments SET status = '已结算' WHERE order_id = $1 AND supplier_code = $2 AND status = '已验收'")
        .bind(body.order_id).bind(&body.supplier_code).execute(pool.get_ref()).await;
    let all_settled = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM carton_shipments WHERE order_id = $1 AND status != '已结算'"
    ).bind(body.order_id).fetch_one(pool.get_ref()).await;
    if let Ok((0,)) = all_settled {
        let _ = sqlx::query("UPDATE carton_orders SET status = '已结算' WHERE id = $1")
            .bind(body.order_id).execute(pool.get_ref()).await;
    }
    HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"message": "结算完成"})))
}
