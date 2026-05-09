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
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(50) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            role VARCHAR(50) NOT NULL DEFAULT 'user',
            avatar VARCHAR(255) NOT NULL DEFAULT '',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create users table");
}

pub async fn seed_admin(pool: &PgPool) {
    let exists: (bool,) =
        sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE username = 'admin')")
            .fetch_one(pool)
            .await
            .expect("Failed to check admin existence");

    if !exists.0 {
        let hash = bcrypt::hash("123456", bcrypt::DEFAULT_COST).expect("Failed to hash password");
        sqlx::query("INSERT INTO users (username, password_hash, role) VALUES ('admin', $1, '管理员')")
            .bind(&hash)
            .execute(pool)
            .await
            .expect("Failed to seed admin user");
        println!("  -> Default admin user created (admin / 123456)");
    }
}
