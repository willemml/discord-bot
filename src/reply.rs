use std::sync::Arc;

use crate::commands::owner_check;
use poise::serenity_prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLockWriteGuard;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Reply {
    pub matches: Vec<String>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReplyConfig {
    pub simple_replies: Vec<Reply>,
}

pub async fn check_and_reply(
    ctx: &Context,
    reply_config: &ReplyConfig,
    msg: Message,
) -> Result<()> {
    for set in &reply_config.simple_replies {
        for trigger in &set.matches {
            if msg
                .content
                .to_lowercase()
                .replace("'", "")
                .contains(trigger)
            {
                msg.channel_id
                    .send_message(&ctx.http(), |m| {
                        m.content(set.text.clone());
                        m.reference_message(&msg)
                    })
                    .await?;
            }
        }
    }
    Ok(())
}

async fn write_config(new_config: ReplyConfig) -> std::result::Result<(), std::io::Error> {
    tokio::fs::write(
        "reply_config.json",
        serde_json::to_string_pretty(&new_config)?,
    )
    .await
}

async fn edit_replies<F: FnOnce(&mut RwLockWriteGuard<ReplyConfig>) -> ()>(
    ctx: crate::Context<'_>,
    edit: F,
) -> std::result::Result<(), std::io::Error> {
    let new_config = {
        let mut replies = ctx.data().reply_config.write().await;
        edit(&mut replies);
        replies.clone()
    };
    write_config(new_config).await
}

#[poise::command(slash_command, check = "owner_check")]
pub async fn new_reply(
    ctx: crate::Context<'_>,
    trigger: String,
    response: String,
) -> std::result::Result<(), crate::Error> {
    edit_replies(ctx, |replies| {
        replies.simple_replies.push(Reply {
            matches: vec![trigger],
            text: response,
        });
    })
    .await?;

    ctx.say("Done.").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn print_reply_sets(ctx: crate::Context<'_>) -> std::result::Result<(), crate::Error> {
    let mut message = String::from("```");
    for (n, set) in (&ctx.data().reply_config.read().await.simple_replies)
        .iter()
        .enumerate()
    {
        let triggers = set.matches.join("\"\n            \"");
        message.push_str(&format!(
            "Set #{}:\n  triggers: \"{}\"\n  response: \"{}\"\n",
            n, triggers, set.text
        ));
    }
    message.push_str("```");
    ctx.say(message).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, check = "owner_check")]
pub async fn delete_reply_set(
    ctx: crate::Context<'_>,
    number: usize,
) -> std::result::Result<(), crate::Error> {
    edit_replies(ctx, |replies| {
        replies.simple_replies.remove(number);
    })
    .await?;

    ctx.say("Done.").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, check = "owner_check")]
pub async fn delete_reply_trigger(
    ctx: crate::Context<'_>,
    set_number: usize,
    trigger: usize,
) -> std::result::Result<(), crate::Error> {
    edit_replies(ctx, |replies| {
        replies.simple_replies[set_number].matches.remove(trigger);
    })
    .await?;

    ctx.say("Done.").await?;
    Ok(())
}

#[poise::command(slash_command, check = "owner_check")]
pub async fn add_reply_trigger(
    ctx: crate::Context<'_>,
    set_number: usize,
    trigger: String,
) -> std::result::Result<(), crate::Error> {
    edit_replies(ctx, |replies| {
        replies.simple_replies[set_number].matches.push(trigger);
    })
    .await?;

    ctx.say("Done.").await?;
    Ok(())
}

#[poise::command(slash_command, check = "owner_check")]
pub async fn change_reply(
    ctx: crate::Context<'_>,
    set_number: usize,
    new_text: String,
) -> std::result::Result<(), crate::Error> {
    edit_replies(ctx, |replies| {
        replies.simple_replies[set_number].text = new_text;
    })
    .await?;

    ctx.say("Done.").await?;
    Ok(())
}
