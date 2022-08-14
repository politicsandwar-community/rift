use sqlx::postgres::PgPoolOptions;

// User data, which is stored and accessible in all command invocations
pub struct Data {
    pub pool: sqlx::PgPool,
    pub cache: super::Cache,
}

impl Data {
    pub async fn new() -> Self {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .connect(&url)
            .await
            .expect("failed to connect to database");
        let cache = super::Cache::hydrate(&pool).await;

        Data { pool, cache }
    }
}
