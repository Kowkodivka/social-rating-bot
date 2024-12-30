use crate::{components::translation::translate, Data, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: poise::Context<'_, Data, Error>) -> Result<(), Error> {
    ctx.reply(translate!(ctx, "ping-message")).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn experience(ctx: poise::Context<'_, Data, Error>) -> Result<(), Error> {
    let user_id = ctx.author().id.get();

    let db = &ctx.data().database;
    let experience = db
        .get_user_experience(user_id.try_into().unwrap())
        .await
        .unwrap_or(Some(0));

    let experience_str = experience.unwrap().to_string();

    ctx.reply(translate!(
        ctx,
        "experience-message",
        experience: &experience_str
    ))
    .await?;

    Ok(())
}
