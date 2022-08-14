mod commands;
mod structs;
mod traits;
mod types;

use poise::serenity_prelude as serenity;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

use crate::structs::Data;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&url)
        .await
        .expect("failed to connect to database");
    let cache = structs::Cache::hydrate(&pool).await;

    let data = Data { pool, cache };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::commands(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(data) }));

    framework.run().await.unwrap();
    println!("STARTED");
}
