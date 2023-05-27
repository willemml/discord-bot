use std::time::Duration;

use crate::checks::*;

use poise::serenity_prelude::CacheHttp;
use rand::Rng;

use crate::{Context, Error};

pub async fn command_response(
    ctx: Context<'_>,
    alternate_message: Option<&str>,
    delete_timeout: Option<u64>,
) -> Result<(), Error> {
    let timeout = delete_timeout.unwrap_or(5000);
    let message = alternate_message.unwrap_or("Done.");
    let reply = ctx.say(message).await?.into_message().await?;
    let http = ctx.serenity_context().http.clone();
    tokio::task::spawn(async move {
        tokio::time::sleep(Duration::from_millis(timeout)).await;
        let _ = reply.delete(&http).await;
    });
    Ok(())
}

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

#[poise::command(slash_command, guild_only, check = "owner_check")]
pub async fn rename(
    ctx: Context<'_>,
    #[description = "My new name"] new_name: String,
) -> Result<(), Error> {
    ctx.guild_id()
        .unwrap()
        .edit_nickname(&ctx.http(), Some(&new_name))
        .await?;
    command_response(ctx, None, None).await
}

#[poise::command(slash_command, prefix_command, check = "owner_check")]
pub async fn delete_message(ctx: Context<'_>, message_id: u64) -> Result<(), Error> {
    let response = if ctx
        .channel_id()
        .delete_message(ctx.http(), message_id)
        .await
        .is_err()
    {
        Some("Failed to delete the message.")
    } else {
        None
    };
    command_response(ctx, response, None).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn dice_roller(
    ctx: Context<'_>,
    #[description = "What kind of dice to roll"]
    #[min = 2]
    #[max = 20]
    size: Option<usize>,
    #[description = "How many dice to roll"]
    #[min = 1]
    #[max = 10]
    count: Option<usize>,
) -> Result<(), Error> {
    let count = count.unwrap_or(1);
    let size = size.unwrap_or(6);

    let mut response = format!("```Results ({}d{}):", count, size);

    {
        let mut rng = rand::thread_rng();

        for _ in 1..=count {
            response.push_str(&format!("\n  - {}", rng.gen_range(1..=size)));
        }
        response.push_str("\n```");
    }

    ctx.say(response).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn xkcd(ctx: Context<'_>, number: Option<usize>) -> Result<(), Error> {
    const XKCD_URL: &str = "https://xkcd.com/";
    if let Some(number) = number {
        ctx.say(format!("{}/{}", XKCD_URL, number)).await?;
    } else {
        ctx.say(XKCD_URL).await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn source_code(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://github.com/willemml/discord-bot").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn author(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("<@237237152495304704> (https://github.com/willemml)")
        .await?;
    Ok(())
}
