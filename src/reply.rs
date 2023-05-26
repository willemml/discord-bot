use poise::serenity_prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Reply {
    matches: Vec<String>,
    text: Option<String>,
    file: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReplyConfig {
    simple_replies: Vec<Reply>,
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
                        if let Some(file) = &set.file {
                            m.add_file(file.as_str());
                        }
                        if let Some(text) = &set.text {
                            m.content(text);
                        }
                        m.reference_message(&msg)
                    })
                    .await?;
            }
        }
    }
    Ok(())
}
