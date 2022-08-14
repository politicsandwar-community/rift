mod commands;
mod structs;
mod traits;
mod types;

use poise::serenity_prelude as serenity;

use dotenv::dotenv;

use crate::structs::Data;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let data = Data::new().await;

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
