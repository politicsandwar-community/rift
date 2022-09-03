use poise::serenity_prelude as serenity;

use crate::types::Command;
use crate::types::{Context, Error};

#[poise::command(slash_command, prefix_command)]
async fn who(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let gotten_user = ctx.data().cache.get_user(&i64::try_from(u.id.0).unwrap());
    let response = format!("test {}", gotten_user.unwrap().nation_id.unwrap());
    ctx.say(response).await?;
    Ok(())
}

pub fn commands() -> [Command; 1] {
    [who()]
}
