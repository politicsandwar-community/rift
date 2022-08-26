use poise::serenity_prelude as serenity;

use crate::types::Command;
use crate::types::{Context, Error};

#[poise::command(slash_command, prefix_command)]
async fn who(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("your name is {}", u.name);
    ctx.say(response).await?;
    Ok(())
}

pub fn commands() -> [Command; 1] {
    [who()]
}
