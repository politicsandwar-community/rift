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

use crate::structs::Data;
use clap::Parser;
use dotenv::dotenv;
use poise::serenity_prelude as serenity;

#[derive(Debug, Parser)]
enum Cli {
    #[command(about = "Run the bot")]
    Run {
        #[arg(short, long = "refresh-from-api")]
        #[arg(default_value = "false")]
        refresh_from_api: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli {
        Cli::Run { refresh_from_api } => {
            dotenv().ok();

            let data = Data::new(refresh_from_api).await;

            let framework = poise::Framework::builder()
                .options(poise::FrameworkOptions {
                    commands: commands::commands(),
                    prefix_options: poise::PrefixFrameworkOptions {
                        prefix: Some("~".into()),
                        case_insensitive_commands: true,
                        ..Default::default()
                    },
                    on_error: |e| Box::pin(errors::on_error(e)),
                    event_handler: |ctx, event, framework, data| {
                        Box::pin(listener::listener(ctx, event, framework, data))
                    },
                    ..Default::default()
                })
                .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
                .intents(
                    serenity::GatewayIntents::non_privileged()
                        | serenity::GatewayIntents::MESSAGE_CONTENT,
                )
                .setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(data) }));

            println!("RUNNING");
            framework.run().await.unwrap();
            println!("STARTED");
        },
    }
}
