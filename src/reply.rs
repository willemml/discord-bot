use crate::checks::*;
use poise::serenity_prelude::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

type CommandResult = std::result::Result<(), crate::Error>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Reply {
    matches: Vec<String>,
    text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct GuildReplyConfig {
    simple_replies: Vec<Reply>,
    timeout: u64,
    auto_reply: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReplyConfig {
    pub guild_configs: HashMap<u64, GuildReplyConfig>,
}

pub async fn check_and_reply(
    ctx: &Context,
    reply_config: &GuildReplyConfig,
    msg: Message,
) -> Result<()> {
    if reply_config.auto_reply {
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

async fn edit_replies<F: FnOnce(&mut GuildReplyConfig) -> ()>(
    ctx: crate::Context<'_>,
    edit: F,
) -> std::result::Result<(), poise::serenity_prelude::Error> {
    if let Some(guild_id) = ctx.guild_id() {
        let new_config = {
            let mut all_replies = ctx.data().reply_config.write().await;
            let replies =
                if let Some(replies) = all_replies.guild_configs.get_mut(guild_id.as_u64()) {
                    replies
                } else {
                    all_replies
                        .guild_configs
                        .insert(*guild_id.as_u64(), GuildReplyConfig::default())
                        .ok_or(Error::Other(
                            "unable to create reply config for current guild",
                        ))?;
                    all_replies
                        .guild_configs
                        .get_mut(guild_id.as_u64())
                        .ok_or(Error::Other(
                            "cannot get new reply config for current guild",
                        ))?
                };
            edit(replies);
            all_replies.clone()
        };

        write_config(new_config).await?;
    } else {
    }
    Ok(())
}

#[poise::command(slash_command, check = "manage_messages_check")]
pub async fn toggle_auto_reply(ctx: crate::Context<'_>, new_state: Option<bool>) -> CommandResult {
    let mut auto_reply = false;
    edit_replies(ctx, |replies| {
        replies.auto_reply = new_state.unwrap_or(!replies.auto_reply);
        auto_reply = replies.auto_reply;
    })
    .await?;

    ctx.say(format!("Auto reply is now {}.", auto_reply)).await?;
    Ok(())
}

#[poise::command(slash_command, check = "manage_messages_check")]
pub async fn change_timeout(ctx: crate::Context<'_>, new_timeout: u64) -> CommandResult {
    edit_replies(ctx, |replies| {
        replies.timeout = new_timeout;
    })
    .await?;

    ctx.say("Done.").await?;
    Ok(())
}

#[poise::command(slash_command, check = "manage_messages_check")]
pub async fn new_reply(
    ctx: crate::Context<'_>,
    trigger: String,
    response: String,
) -> CommandResult {
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
pub async fn print_reply_sets(ctx: crate::Context<'_>) -> CommandResult {
    let mut message = String::from("```");

    for (n, set) in (&ctx
        .data()
        .reply_config
        .read()
        .await
        .guild_configs
        .get(ctx.guild_id().ok_or(Error::Other("not in guild"))?.as_u64())
        .unwrap_or(&GuildReplyConfig::default())
        .simple_replies)
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

#[poise::command(slash_command, prefix_command, check = "manage_messages_check")]
pub async fn delete_reply_set(ctx: crate::Context<'_>, number: usize) -> CommandResult {
    edit_replies(ctx, |replies| {
        replies.simple_replies.remove(number);
    })
    .await?;

    ctx.say("Done.").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, check = "manage_messages_check")]
pub async fn delete_reply_trigger(
    ctx: crate::Context<'_>,
    set_number: usize,
    trigger: usize,
) -> CommandResult {
    edit_replies(ctx, |replies| {
        replies.simple_replies[set_number].matches.remove(trigger);
    })
    .await?;

    ctx.say("Done.").await?;
    Ok(())
}

#[poise::command(slash_command, check = "manage_messages_check")]
pub async fn add_reply_trigger(
    ctx: crate::Context<'_>,
    set_number: usize,
    trigger: String,
) -> CommandResult {
    edit_replies(ctx, |replies| {
        replies.simple_replies[set_number].matches.push(trigger);
    })
    .await?;

    ctx.say("Done.").await?;
    Ok(())
}

#[poise::command(slash_command, check = "manage_messages_check")]
pub async fn change_reply(
    ctx: crate::Context<'_>,
    set_number: usize,
    new_text: String,
) -> CommandResult {
    edit_replies(ctx, |replies| {
        replies.simple_replies[set_number].text = new_text;
    })
    .await?;

    ctx.say("Done.").await?;
    Ok(())
}
