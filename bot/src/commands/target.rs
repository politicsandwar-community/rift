use crate::{
    convert,
    structs::TargetRater,
    types::{Command, Context, Error},
};

#[poise::command(
    slash_command,
    subcommands("target_find", "target_config", "target_rater")
)]
async fn target(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "find")]
async fn target_find(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "config")]
async fn target_config(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    slash_command,
    rename = "rater",
    subcommands(
        "target_rater_info",
        "target_rater_create",
        "target_rater_delete",
        "target_rater_edit",
        "target_rater_list"
    )
)]
async fn target_rater(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "info")]
async fn target_rater_info(
    ctx: Context<'_>,
    #[description = "The target rater to use."] rater: String,
) -> Result<(), Error> {
    convert!(ctx, rater = TargetRater);
    Ok(())
}

#[poise::command(slash_command, rename = "create")]
async fn target_rater_create(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "delete")]
async fn target_rater_delete(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "edit")]
async fn target_rater_edit(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "list")]
async fn target_rater_list(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

pub fn commands() -> [Command; 1] {
    [target()]
}
