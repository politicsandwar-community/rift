use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;

use crate::traits::Model;

#[derive(Clone, Debug)]
pub struct Data {
    pub pool: Arc<sqlx::PgPool>,
    pub cache: Arc<super::Cache>,
    pub kit: Arc<pnwkit::Kit>,
}

impl Data {
    pub async fn new(refresh_from_api: bool) -> Self {
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

        let data = Data { pool, cache, kit };

        data.cache.start_subscriptions(&data).await;
        if refresh_from_api {
            let d = data.clone();
            tokio::spawn(async move {
                d.cache.refresh_from_api(&d).await;
            });
        }

        let d = data.clone();
        tokio::spawn(async move {
            let sub = d
                .kit
                .subscribe(
                    pnwkit::SubscriptionModel::Account,
                    pnwkit::SubscriptionEvent::Update,
                )
                .await
                .expect("subscription failed");
            while let Some(obj) = sub.next().await {
                let data = d.clone();
                tokio::spawn(async move {
                    let value = data
                        .cache
                        .get_nation(&obj.get("id").unwrap().value().as_i32().unwrap());
                    if let Some(mut value) = value {
                        let _lock = value.lock(&data).await;
                        value.last_active = obj.get("last_active").unwrap().value().into();
                        value.discord_id = obj.get("discord_id").unwrap().value().into();
                        if let Err(e) = value.save(&data, false).await {
                            panic!("error saving object: {}", e);
                        }
                    }
                });
            }
        });

        data
    }
}
