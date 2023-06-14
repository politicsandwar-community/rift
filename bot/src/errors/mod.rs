#![allow(unused_must_use)]
#![allow(unused_variables)]
mod not_found;

pub use not_found::*;
use poise::FrameworkError;

use crate::{embeds, structs::Data, types::Error};

pub async fn on_error(e: FrameworkError<'_, Data, Error>) {
    match e {
        FrameworkError::ArgumentParse { error, input, ctx } => {
            ctx.send(|f| f.embed(embeds::fatal_error(&ctx))).await;
        },
        FrameworkError::Command { error, ctx } => {
            ctx.send(|f| f.embed(error.to_embed(&ctx))).await;
        },
        FrameworkError::CommandCheckFailed {
            error: Some(error),
            ctx,
        } => {
            ctx.send(|f| f.embed(error.to_embed(&ctx))).await;
        },
        FrameworkError::CommandStructureMismatch { description, ctx } => {},
        FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
        } => {},
        FrameworkError::DmOnly { ctx } => {
            ctx.send(|f| f.embed(embeds::dm_only_error(&ctx))).await;
        },
        FrameworkError::DynamicPrefix { error, .. } => {},
        FrameworkError::GuildOnly { ctx } => {
            ctx.send(|r| r.embed(embeds::guild_only_error(&ctx))).await;
        },
        FrameworkError::EventHandler {
            error,
            ctx,
            event,
            framework,
        } => {},
        FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
        } => {},
        FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
        } => {},
        FrameworkError::NotAnOwner { ctx } => {},
        FrameworkError::NsfwOnly { ctx } => {},
        FrameworkError::Setup { error, .. } => {
            panic!("{}", error);
        },
        _ => {},
    }
}
