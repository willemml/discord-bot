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
            extra_text_at_bottom:
                "This is a simple bot made using the Rust Poise framework for Discord.",
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
    size: Option<usize>,
    count: Option<usize>,
) -> Result<(), Error> {
    let mut count = count.unwrap_or(1);
    let mut size = size.unwrap_or(6);
    if size < 1 {
        ctx.say("Cannot divide by zero.").await?;
        size = 2;
    }
    if count > 10 {
        ctx.say("The maximum number of dice you can roll at once is 10, rolling 10 dice...")
            .await?;
        count = 10;
    } else if count == 0 {
        ctx.say("You must roll at least one die.").await?;
        count = 1;
    }

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
pub async fn xkcd(ctx: Context<'_>, number: usize) -> Result<(), Error> {
    ctx.say(format!("https://xkcd.com/{}", number)).await?;
    Ok(())
}
