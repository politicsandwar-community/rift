use crate::types::Command;
use crate::types::{Context, Error};

#[poise::command(slash_command, prefix_command)]
async fn toot(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Toot tooooooot!").await?;
    Ok(())
}

pub fn commands() -> [Command; 1] {
    [toot()]
}
