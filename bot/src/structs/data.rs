use std::{sync::Arc, time::Duration};

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

        let _lock = cache.lock_nation(&251584).await;
        // drop(_lock);
        let timeout = tokio::time::timeout(Duration::from_secs(5), async {
            cache.lock_nation(&251584).await
        })
        .await;
        match timeout {
            Ok(_) => panic!("locked!"),
            Err(_) => panic!("didn't lock!"),
        }

        let data = Data { pool, cache, kit };

        data.cache.start_subscriptions(&data);

        data
    }
}
