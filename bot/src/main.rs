mod checks;
mod commands;
mod components;
mod consts;
mod embeds;
mod enums;
mod errors;
mod listener;
mod strings;
mod structs;
mod traits;
mod types;
mod utils;

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
            on_error: |e| Box::pin(errors::on_error(e)),
            listener: |ctx, event, framework, data| {
                Box::pin(listener::listener(ctx, event, framework, data))
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
