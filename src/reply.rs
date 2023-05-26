use crate::commands::owner_check;
use poise::serenity_prelude::*;
use serde::{Deserialize, Serialize};

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

#[poise::command(slash_command, check = "owner_check")]
pub async fn new_reply(
    ctx: crate::Context<'_>,
    trigger: String,
    response: String,
) -> std::result::Result<(), crate::Error> {
    let new_config = {
        let mut replies = ctx.data().reply_config.write().await;
        replies.simple_replies.push(Reply {
            matches: vec![trigger],
            text: response,
        });
        replies.clone()
    };

    tokio::fs::write(
        "reply_config.json",
        serde_json::to_string_pretty(&new_config)?,
    )
    .await?;

    ctx.say("Trigger response pair registered.").await?;
    Ok(())
}
