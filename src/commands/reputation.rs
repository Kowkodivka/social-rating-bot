use crate::{components::translation::translate, Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(context_menu_command = "Repute", prefix_command, slash_command)]
pub async fn repute(ctx: Context<'_>, msg: serenity::Message) -> Result<(), Error> {
    let db = &ctx.data().database;

    let author_id = ctx.author().id.get() as i64;
    let message_id = msg.id.get() as i64;
    let user_id = msg.author.id.get() as i64;

    let experience = db.get_user_experience(author_id).await.unwrap_or(Some(0));
    if experience.unwrap_or(0) < 100 {
        ctx.send(
            poise::CreateReply::default()
                .content(translate!(
                    ctx,
                    "not-enough-experience",
                    required_experience: 100
                ))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    match db
        .set_message_reputation(user_id, message_id, author_id, 1)
        .await
    {
        Ok(_) => {
            ctx.send(
                poise::CreateReply::default()
                    .content(translate!(ctx, "reputation-added"))
                    .ephemeral(true),
            )
            .await?;
        }
        Err(_) => {
            ctx.send(
                poise::CreateReply::default()
                    .content(translate!(ctx, "reputation-already-given"))
                    .ephemeral(true),
            )
            .await?;
        }
    }

    Ok(())
}

#[poise::command(context_menu_command = "Diminish", prefix_command, slash_command)]
pub async fn diminish(ctx: Context<'_>, msg: serenity::Message) -> Result<(), Error> {
    let db = &ctx.data().database;

    let author_id = ctx.author().id.get() as i64;
    let message_id = msg.id.get() as i64;
    let user_id = msg.author.id.get() as i64;

    match db
        .remove_message_reputation(user_id, message_id, author_id)
        .await
    {
        Ok(_) => {
            ctx.send(
                poise::CreateReply::default()
                    .content(translate!(ctx, "reputation-removed"))
                    .ephemeral(true),
            )
            .await?;
        }
        Err(_) => {
            ctx.send(
                poise::CreateReply::default()
                    .content(translate!(ctx, "reputation-not-found"))
                    .ephemeral(true),
            )
            .await?;
        }
    }

    Ok(())
}

#[poise::command(
    context_menu_command = "Show message reputation",
    prefix_command,
    slash_command
)]
pub async fn show_message_reputation(
    ctx: Context<'_>,
    msg: serenity::Message,
) -> Result<(), Error> {
    let db = &ctx.data().database;

    let message_id = msg.id.get() as i64;
    let reputation = db
        .get_message_reputation(message_id)
        .await
        .unwrap_or(Some(0));

    ctx.send(
        poise::CreateReply::default()
            .content(translate!(
                ctx,
                "message-reputation",
                message_content: &msg.content,
                reputation: reputation.unwrap_or(0)
            ))
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

#[poise::command(
    context_menu_command = "Show user reputation",
    prefix_command,
    slash_command
)]
pub async fn show_user_reputation(ctx: Context<'_>, user: serenity::User) -> Result<(), Error> {
    let db = &ctx.data().database;

    let user_id = user.id.get() as i64;

    let reputation = db
        .get_user_global_reputation(user_id)
        .await
        .unwrap_or(Some(0));

    ctx.send(
        poise::CreateReply::default()
            .content(translate!(
                ctx,
                "user-reputation",
                user_name: &user.name,
                reputation: reputation.unwrap_or(0)
            ))
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
