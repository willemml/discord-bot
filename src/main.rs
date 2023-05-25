use std::env;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

const REPLIES: &[(&[&str], Option<&str>, Option<&str>)] = &[
    (&["give me"], Some("no-way-dude-no.gif"), None),
    (
        &["too many keys", "the keys are overly plentiful"],
        Some("too-many-keys.png"),
        None,
    ),
    (
        &["huh"],
        None,
        Some("https://tenor.com/view/huh-huh-meme-huh-gif-simpsons-meme-simpsons-gif-24537736"),
    ),
];

#[group]
#[commands(ping, name_yourself)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot {
            for set in REPLIES {
                for trigger in set.0 {
                    if msg.content.to_lowercase().contains(trigger) {
                        let _ = msg
                            .channel_id
                            .send_message(&ctx.http, |m| {
                                if let Some(file) = set.1 {
                                    m.add_file(file);
                                }
                                if let Some(text) = set.2 {
                                    m.content(text);
                                }
                                m.reference_message(&msg)
                            })
                            .await;
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn name_yourself(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Some(guild_id) = msg.guild_id {
        guild_id
            .edit_nickname(&ctx.http, Some(&format!("{}", args.rest())))
            .await?;
    }
    msg.channel_id
        .send_message(&ctx.http, |m| m.content("Your wish is my command..."))
        .await?;
    Ok(())
}
