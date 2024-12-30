use crate::{components::translation::translate, Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply(translate!(ctx, "ping-message")).await?;
    Ok(())
}
