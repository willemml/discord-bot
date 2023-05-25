use poise::serenity_prelude::CacheHttp;

use crate::{Context, Error};

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is a simple bot made using the Rust Poise framework for Discord.",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn rename(ctx: Context<'_>, #[description = "My new name"] new_name: String) -> Result<(), Error> {
    if let Some(guild_id) = ctx.guild_id() {
        guild_id.edit_nickname(&ctx.http(), Some(&new_name)).await?;
        ctx.say("Your wish is my command...").await?;
    }

    Ok(())
}
