#![warn(clippy::str_to_string)]

mod checks;
mod commands;
mod reply;

use poise::{serenity_prelude as serenity, Event};
use std::{env::var, sync::Arc, time::Duration};
use tokio::sync::RwLock;

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    reply_config: Arc<RwLock<reply::ReplyConfig>>,
    owner: u64,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let reply_config_file =
        std::fs::File::open("reply_config.json").expect("Reply configuration is missing...");

    let reply_config = Arc::new(RwLock::new(
        serde_json::from_reader(reply_config_file).expect("Bad reply configuration."),
    ));

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::dice_roller(),
            commands::help(),
            commands::ping(),
            commands::rename(),
            commands::xkcd(),
            reply::add_reply_trigger(),
            reply::change_reply(),
            reply::change_timeout(),
            reply::delete_reply_set(),
            reply::delete_reply_trigger(),
            reply::new_reply(),
            reply::print_reply_sets(),
            reply::toggle_auto_reply(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            ..Default::default()
        },
        /// The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        /// This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        /// This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        skip_checks_for_owners: true,
        event_handler: |ctx, event, _framework, data| {
            Box::pin(async move {
                match event.clone() {
                    Event::Message { new_message } => {
                        if !new_message.author.bot {
                            if let Some(guild_id) = new_message.guild_id {
                                if let Some(guild_reply_config) = {
                                    let all_config = data.reply_config.read().await;
                                    all_config.guild_configs.get(guild_id.as_u64()).cloned()
                                } {
                                    reply::check_and_reply(ctx, &guild_reply_config, new_message)
                                        .await?;
                                }
                            }
                        }
                    }
                    _ => {}
                }
                Ok(())
            })
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .token(var("DISCORD_TOKEN").expect("Missing `DISCORD_TOKEN` env var."))
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    reply_config,
                    owner: 237237152495304704,
                })
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .run()
        .await
        .unwrap();
}
