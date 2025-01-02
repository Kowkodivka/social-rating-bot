use crate::{components::translation::translate, Context, Error};

#[poise::command(prefix_command, slash_command, subcommands("view", "leaderboard"))]
pub async fn experience(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply(translate!(ctx, "experience-main-message"))
        .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn view(ctx: Context<'_>) -> Result<(), Error> {
    let user_id = ctx.author().id.get();

    let db = &ctx.data().database;
    let experience = db
        .fetch_user_experience(user_id.try_into().unwrap())
        .await
        .unwrap_or(Some(0));

    let experience_value = experience.unwrap_or(0);

    ctx.reply(translate!(
        ctx,
        "experience-message",
        experience: &experience_value
    ))
    .await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let db = &ctx.data().database;

    let leaders = db.fetch_leaderboard(10).await.unwrap_or_default();

    if leaders.is_empty() {
        ctx.reply(translate!(ctx, "leaderboard-empty")).await?;
        return Ok(());
    }

    let leaderboard = leaders
        .into_iter()
        .enumerate()
        .map(|(index, (user_id, experience))| {
            translate!(
                ctx,
                "leaderboard-entry",
                position: index + 1,
                user_id: user_id,
                experience: experience
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    ctx.reply(translate!(
        ctx,
        "leaderboard-message",
        leaderboard: &leaderboard
    ))
    .await?;

    Ok(())
}
