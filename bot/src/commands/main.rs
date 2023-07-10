use poise::serenity_prelude as serenity;

use crate::{
    structs::{Alliance, City, Nation, Tradeprice},
    traits::Model,
    types::{Command, Context, Error},
};

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[poise::command(prefix_command)]
async fn load_data(ctx: Context<'_>) -> Result<(), Error> {
    if !ctx.framework().options().owners.contains(&ctx.author().id) {
        ctx.say("Only the bot owner can load the data").await?;
    } else {
        // Nation::refresh_from_api(&ctx.data()).await?;
        // Alliance::refresh_from_api(&ctx.data()).await?;
        // City::refresh_from_api(&ctx.data()).await?;
        Tradeprice::refresh_from_api(&ctx.data()).await?;
        ctx.say("Done").await?;
    };

    Ok(())
}

pub fn commands() -> [Command; 3] {
    [age(), register(), load_data()]
}
