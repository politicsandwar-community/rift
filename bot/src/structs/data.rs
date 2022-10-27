use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct Data {
    pub pool: Arc<sqlx::PgPool>,
    pub cache: Arc<super::Cache>,
    pub kit: Arc<pnwkit::Kit>,
}

impl Data {
    pub async fn new() -> Self {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = Arc::new(
            PgPoolOptions::new()
                .connect(&url)
                .await
                .expect("failed to connect to database"),
        );
        let cache = Arc::new(super::Cache::hydrate(&pool).await);
        let kit = Arc::new(
            pnwkit::Config::new()
                .set_api_key(std::env::var("API_KEY").expect("API_KEY must be set"))
                .to_kit(),
        );

        Data { pool, cache, kit }
    }
}
