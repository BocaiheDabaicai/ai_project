use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn create_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL")
}

pub async fn run_migrations(pool: &PgPool) {
    // 用户表
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY, username VARCHAR(50) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL, role VARCHAR(50) NOT NULL DEFAULT 'user',
            avatar VARCHAR(255) DEFAULT '', created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW())"#,
    ).execute(pool).await.expect("Failed to create users table");

    // 纸箱请购单
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS carton_requisitions (
            id SERIAL PRIMARY KEY, req_no VARCHAR(50) UNIQUE NOT NULL,
            dept VARCHAR(100), applicant VARCHAR(50), total_qty INTEGER NOT NULL DEFAULT 0,
            status VARCHAR(20) DEFAULT '待处理', push_time TIMESTAMPTZ DEFAULT NOW(),
            created_at TIMESTAMPTZ DEFAULT NOW())"#,
    ).execute(pool).await.expect("Failed to create carton_requisitions");

    // 请购单物料
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS carton_requisition_items (
            id SERIAL PRIMARY KEY, req_id INTEGER REFERENCES carton_requisitions(id),
            material_code VARCHAR(50), material_name VARCHAR(200), spec VARCHAR(200),
            qty INTEGER NOT NULL DEFAULT 0, unit VARCHAR(20) DEFAULT '个',
            is_new_product BOOLEAN DEFAULT false, oa_price DECIMAL(10,2))"#,
    ).execute(pool).await.expect("Failed to create carton_requisition_items");

    // 纸箱采购订单
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS carton_orders (
            id SERIAL PRIMARY KEY, order_no VARCHAR(50) UNIQUE NOT NULL,
            req_id INTEGER, req_no VARCHAR(50), name VARCHAR(200),
            total_qty INTEGER DEFAULT 0, total_delivered INTEGER DEFAULT 0,
            total_amount DECIMAL(12,2) DEFAULT 0, status VARCHAR(20) DEFAULT '待审批',
            approved BOOLEAN DEFAULT false, approved_by VARCHAR(50),
            approved_at TIMESTAMPTZ, created_at TIMESTAMPTZ DEFAULT NOW())"#,
    ).execute(pool).await.expect("Failed to create carton_orders");

    // 订单供应商明细
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS carton_order_suppliers (
            id SERIAL PRIMARY KEY, order_id INTEGER REFERENCES carton_orders(id),
            material_code VARCHAR(50), material_name VARCHAR(200),
            supplier_code VARCHAR(50), supplier_name VARCHAR(200),
            qty INTEGER DEFAULT 0, delivered INTEGER DEFAULT 0,
            unit_price DECIMAL(10,2), pkg VARCHAR(50), strategy VARCHAR(20),
            card_paper VARCHAR(200), test_result VARCHAR(20), color_diff VARCHAR(20),
            quality VARCHAR(20), score DECIMAL(3,1) DEFAULT 0,
            is_standard_sample BOOLEAN DEFAULT false, has_bulk_history BOOLEAN DEFAULT false,
            production_location VARCHAR(100), notes TEXT)"#,
    ).execute(pool).await.expect("Failed to create carton_order_suppliers");

    // 发货批次
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS carton_shipments (
            id SERIAL PRIMARY KEY, order_id INTEGER REFERENCES carton_orders(id),
            batch_no VARCHAR(50), material_code VARCHAR(50), material_name VARCHAR(200),
            supplier_code VARCHAR(50), supplier_name VARCHAR(200),
            qty INTEGER DEFAULT 0, unit VARCHAR(20), cost DECIMAL(10,2),
            status VARCHAR(20) DEFAULT '已下单', arrival_time TIMESTAMPTZ,
            created_at TIMESTAMPTZ DEFAULT NOW())"#,
    ).execute(pool).await.expect("Failed to create carton_shipments");

    // 供应商季度报价
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS carton_supplier_quotes (
            id SERIAL PRIMARY KEY, supplier_code VARCHAR(50), supplier_name VARCHAR(200),
            material_code VARCHAR(50), material_name VARCHAR(200),
            quarter VARCHAR(10), price DECIMAL(10,2), pkg VARCHAR(50))"#,
    ).execute(pool).await.expect("Failed to create carton_supplier_quotes");

    println!("  -> Database migrations complete");
}

pub async fn seed_admin(pool: &PgPool) {
    let exists: (bool,) =
        sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE username = 'admin')")
            .fetch_one(pool).await.expect("Failed to check admin existence");
    if !exists.0 {
        let hash = bcrypt::hash("123456", bcrypt::DEFAULT_COST).expect("Failed to hash password");
        sqlx::query("INSERT INTO users (username, password_hash, role) VALUES ('admin', $1, '管理员')")
            .bind(&hash).execute(pool).await.expect("Failed to seed admin user");
        println!("  -> Default admin user created (admin / 123456)");
    }
}

pub async fn seed_carton_data(pool: &PgPool) {
    // 种子请购单
    let req_exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM carton_requisitions WHERE req_no = 'REQ20260514001')"
    ).fetch_one(pool).await.unwrap_or((false,));
    if !req_exists.0 {
        sqlx::query("INSERT INTO carton_requisitions (req_no, dept, applicant, total_qty, status, push_time) VALUES ('REQ20260514001', '生产部', '赵工', 85000, '待处理', NOW())")
            .execute(pool).await.unwrap();
        let req_id: (i32,) = sqlx::query_as("SELECT id FROM carton_requisitions WHERE req_no = 'REQ20260514001'").fetch_one(pool).await.unwrap();
        let items = vec![
            ("CTN-001", "三层瓦楞纸箱 400x300x250mm", "400×300×250mm B楞", 30000, "个", false, None),
            ("CTN-002", "五层瓦楞纸箱 600x400x350mm", "600×400×350mm BC楞", 25000, "个", false, None),
            ("CTN-003", "重型纸箱 800x600x500mm", "800×600×500mm 三A楞", 15000, "个", true, Some(12.80)),
            ("CTN-004", "食品级纸箱 500x350x300mm", "500×350×300mm E楞 食品级", 10000, "个", false, None),
            ("CTN-005", "防水纸箱 450x300x280mm", "450×300×280mm 防水处理", 5000, "个", true, Some(9.50)),
        ];
        for (code, name, spec, qty, unit, np, oa) in &items {
            sqlx::query("INSERT INTO carton_requisition_items (req_id, material_code, material_name, spec, qty, unit, is_new_product, oa_price) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)")
                .bind(req_id.0).bind(code).bind(name).bind(spec).bind(qty).bind(unit).bind(np).bind(oa)
                .execute(pool).await.unwrap();
        }
    }

    // 种子供应商报价
    let quote_exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM carton_supplier_quotes)")
        .fetch_one(pool).await.unwrap_or((false,));
    if !quote_exists.0 {
        let quotes = vec![
            ("SUP001", "浙江优质纸箱有限公司", "CTN-001", "三层瓦楞纸箱 400x300x250mm", "2026Q2", 3.20, "标包A"),
            ("SUP002", "广州精工包装有限公司", "CTN-001", "三层瓦楞纸箱 400x300x250mm", "2026Q2", 3.05, "标包A"),
            ("SUP003", "上海环保纸品有限公司", "CTN-001", "三层瓦楞纸箱 400x300x250mm", "2026Q2", 3.35, "标包A"),
            ("SUP001", "浙江优质纸箱有限公司", "CTN-002", "五层瓦楞纸箱 600x400x350mm", "2026Q2", 5.80, "标包A"),
            ("SUP002", "广州精工包装有限公司", "CTN-002", "五层瓦楞纸箱 600x400x350mm", "2026Q2", 5.60, "标包A"),
            ("SUP003", "上海环保纸品有限公司", "CTN-002", "五层瓦楞纸箱 600x400x350mm", "2026Q2", 6.00, "标包A"),
            ("SUP004", "北京新材料包装有限公司", "CTN-003", "重型纸箱 800x600x500mm", "2026Q2", 12.50, "标包B"),
            ("SUP005", "深圳精密包装有限公司", "CTN-003", "重型纸箱 800x600x500mm", "2026Q2", 13.00, "标包B"),
            ("SUP001", "浙江优质纸箱有限公司", "CTN-004", "食品级纸箱 500x350x300mm", "2026Q2", 4.50, "标包A"),
            ("SUP002", "广州精工包装有限公司", "CTN-004", "食品级纸箱 500x350x300mm", "2026Q2", 4.80, "标包A"),
            ("SUP004", "北京新材料包装有限公司", "CTN-005", "防水纸箱 450x300x280mm", "2026Q2", 9.00, "标包B"),
            ("SUP005", "深圳精密包装有限公司", "CTN-005", "防水纸箱 450x300x280mm", "2026Q2", 8.80, "标包B"),
        ];
        for (sc, sn, mc, mn, q, p, pkg) in &quotes {
            sqlx::query("INSERT INTO carton_supplier_quotes (supplier_code, supplier_name, material_code, material_name, quarter, price, pkg) VALUES ($1,$2,$3,$4,$5,$6,$7)")
                .bind(sc).bind(sn).bind(mc).bind(mn).bind(q).bind(p).bind(pkg)
                .execute(pool).await.unwrap();
        }
    }

    // 种子订单
    let ord_exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM carton_orders)")
        .fetch_one(pool).await.unwrap_or((false,));
    if !ord_exists.0 {
        sqlx::query("INSERT INTO carton_orders (order_no, req_no, req_id, name, total_qty, total_delivered, total_amount, status, approved) VALUES ('PO20260514001','REQ20260514001',1,'纸箱批量采购',50000,12000,168000.00,'已下单',true)")
            .execute(pool).await.unwrap();
        let ord_id: (i32,) = sqlx::query_as("SELECT id FROM carton_orders WHERE order_no = 'PO20260514001'").fetch_one(pool).await.unwrap();
        let suppliers = vec![
            ("CTN-001","三层瓦楞纸箱 400x300x250mm","SUP001","浙江优质纸箱有限公司",20000,5000,3.20,"标包A","最低价","合格","轻微色差","良好",4.5,true,true,"浙江杭州",""),
            ("CTN-001","三层瓦楞纸箱 400x300x250mm","SUP002","广州精工包装有限公司",10000,3000,3.05,"标包A","最低价","合格","无色差","非常好",4.8,false,true,"广东广州",""),
            ("CTN-002","五层瓦楞纸箱 600x400x350mm","SUP003","上海环保纸品有限公司",15000,2000,6.00,"标包A","自定义","未检测","一般色差","一般",3.5,true,false,"上海",""),
            ("CTN-002","五层瓦楞纸箱 600x400x350mm","SUP001","浙江优质纸箱有限公司",10000,2000,5.80,"标包A","自定义","合格","轻微色差","良好",4.5,true,true,"浙江杭州",""),
        ];
        for (mc, mn, sc, sn, qty, del, up, pkg, s, tr, cd, ql, score, ss, bh, loc, notes) in &suppliers {
            sqlx::query("INSERT INTO carton_order_suppliers (order_id,material_code,material_name,supplier_code,supplier_name,qty,delivered,unit_price,pkg,strategy,test_result,color_diff,quality,score,is_standard_sample,has_bulk_history,production_location,notes) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18)")
                .bind(ord_id.0).bind(mc).bind(mn).bind(sc).bind(sn).bind(qty).bind(del).bind(up).bind(pkg).bind(s).bind(tr).bind(cd).bind(ql).bind(score).bind(ss).bind(bh).bind(loc).bind(notes)
                .execute(pool).await.unwrap();
        }
        // 种子批次
        sqlx::query("INSERT INTO carton_shipments (order_id, batch_no, material_code, material_name, supplier_code, supplier_name, qty, unit, cost, status) VALUES ($1,'BATCH-001','CTN-001','三层瓦楞纸箱 400x300x250mm','SUP001','浙江优质纸箱有限公司',5000,'个',16000.00,'已验收')")
            .bind(ord_id.0).execute(pool).await.unwrap();
        sqlx::query("INSERT INTO carton_shipments (order_id, batch_no, material_code, material_name, supplier_code, supplier_name, qty, unit, cost, status) VALUES ($1,'BATCH-002','CTN-001','三层瓦楞纸箱 400x300x250mm','SUP002','广州精工包装有限公司',3000,'个',9150.00,'已入库')")
            .bind(ord_id.0).execute(pool).await.unwrap();
        sqlx::query("INSERT INTO carton_shipments (order_id, batch_no, material_code, material_name, supplier_code, supplier_name, qty, unit, cost, status) VALUES ($1,'BATCH-003','CTN-002','五层瓦楞纸箱 600x400x350mm','SUP001','浙江优质纸箱有限公司',2000,'个',11600.00,'已发货')")
            .bind(ord_id.0).execute(pool).await.unwrap();
        sqlx::query("INSERT INTO carton_shipments (order_id, batch_no, material_code, material_name, supplier_code, supplier_name, qty, unit, cost, status) VALUES ($1,'BATCH-004','CTN-002','五层瓦楞纸箱 600x400x350mm','SUP003','上海环保纸品有限公司',2000,'个',12000.00,'已下单')")
            .bind(ord_id.0).execute(pool).await.unwrap();
    }
    println!("  -> Carton demo data seeded");
}
