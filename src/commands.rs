use crate::checks::*;
use poise::serenity_prelude::CacheHttp;

use rand::Rng;

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

#[poise::command(slash_command, check = "owner_check")]
pub async fn rename(
    ctx: Context<'_>,
    #[description = "My new name"] new_name: String,
) -> Result<(), Error> {
    if let Some(guild_id) = ctx.guild_id() {
        guild_id.edit_nickname(&ctx.http(), Some(&new_name)).await?;
        ctx.say("Your wish is my command...").await?;
    }

    Ok(())
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
