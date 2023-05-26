use crate::{Context, Error};

pub async fn owner_check(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(if ctx.author().id.0 == ctx.data().owner {
        true
    } else {
        ctx.say("Only the bot owner (wnuke) can use this command.")
            .await?;
        false
    })
}

pub async fn manage_messages_check(ctx: Context<'_>) -> Result<bool, Error> {
    if let Some(author_member) = ctx.author_member().await {
        if let Some(permissions) = author_member.permissions {
            return Ok(permissions.manage_messages());
        } else {
            ctx.say("You do not have permission to use this command.")
                .await?;
            return Err(Error::from("no permissions"));
        }
    } else {
        ctx.say("This command can only be used in a Guild.").await?;
        return Err(Error::from("not in guild"));
    }
}
