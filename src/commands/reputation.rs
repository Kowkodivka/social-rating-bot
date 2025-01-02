use crate::{components::translation::translate, Context, Error};
use poise::serenity_prelude as serenity;

async fn check_experience(ctx: Context<'_>, required_experience: i64) -> Result<bool, Error> {
    let db = &ctx.data().database;
    let author_id = ctx.author().id.get() as i64;
    let experience = db.fetch_user_experience(author_id).await.unwrap_or(Some(0));

    if experience.unwrap_or(0) < required_experience {
        ctx.send(
            poise::CreateReply::default()
                .content(translate!(
                    ctx,
                    "not-enough-experience",
                    required_experience: required_experience
                ))
                .ephemeral(true),
        )
        .await?;

        return Ok(false);
    }

    Ok(true)
}

async fn handle_reputation(
    ctx: Context<'_>,
    msg: serenity::Message,
    reputation_change: i64,
    success_key: &str,
    already_given_key: &str,
) -> Result<(), Error> {
    let db = &ctx.data().database;

    if !check_experience(ctx, 100).await? {
        return Ok(());
    }

    let author_id = ctx.author().id.get() as i64;
    let message_id = msg.id.get() as i64;
    let user_id = msg.author.id.get() as i64;

    match db
        .add_message_reputation(user_id, message_id, author_id, reputation_change)
        .await
    {
        Ok(_) => {
            ctx.send(
                poise::CreateReply::default()
                    .content(translate!(ctx, success_key))
                    .ephemeral(true),
            )
            .await?;
        }
        Err(_) => {
            ctx.send(
                poise::CreateReply::default()
                    .content(translate!(ctx, already_given_key))
                    .ephemeral(true),
            )
            .await?;
        }
    }

    Ok(())
}

#[poise::command(context_menu_command = "Repute")]
pub async fn repute(ctx: Context<'_>, msg: serenity::Message) -> Result<(), Error> {
    handle_reputation(ctx, msg, 1, "reputation-added", "reputation-already-given").await
}

#[poise::command(context_menu_command = "Repute (reverse)")]
pub async fn reverse_repute(ctx: Context<'_>, msg: serenity::Message) -> Result<(), Error> {
    handle_reputation(
        ctx,
        msg,
        -1,
        "reputation-decreased",
        "reputation-already-given",
    )
    .await
}

async fn send_reputation_info(
    ctx: Context<'_>,
    content_key: &str,
    target_name: &str,
    reputation: i64,
) -> Result<(), Error> {
    ctx.send(
        poise::CreateReply::default()
            .content(translate!(
                ctx,
                content_key,
                target_name: target_name,
                reputation: reputation
            ))
            .ephemeral(true),
    )
    .await?;
    Ok(())
}

#[poise::command(context_menu_command = "Show message reputation")]
pub async fn show_message_reputation(
    ctx: Context<'_>,
    msg: serenity::Message,
) -> Result<(), Error> {
    let db = &ctx.data().database;
    let message_id = msg.id.get() as i64;
    let reputation = db
        .fetch_message_reputation(message_id)
        .await
        .unwrap_or(Some(0));
    send_reputation_info(
        ctx,
        "message-reputation",
        &msg.content,
        reputation.unwrap_or(0),
    )
    .await
}

#[poise::command(context_menu_command = "Show user reputation")]
pub async fn show_user_reputation(ctx: Context<'_>, user: serenity::User) -> Result<(), Error> {
    let db = &ctx.data().database;
    let user_id = user.id.get() as i64;
    let reputation = db
        .fetch_user_global_reputation(user_id)
        .await
        .unwrap_or(Some(0));
    send_reputation_info(ctx, "user-reputation", &user.name, reputation.unwrap_or(0)).await
}
