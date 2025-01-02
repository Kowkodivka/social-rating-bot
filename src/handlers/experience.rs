use std::time::Instant;

use poise::serenity_prelude as serenity;

use crate::{Data, Error};

pub async fn experience_message_handler(
    data: &Data,
    message: &serenity::Message,
) -> Result<(), Error> {
    let mut timestamps = data.timestamps.lock().await;

    let user_id = message.author.id.get();

    let should_add_experience = match timestamps.get(&user_id) {
        Some(last_message_time) => {
            last_message_time.elapsed().as_secs() > data.config.experience.message_cooldown_seconds
        }
        None => true,
    };

    if should_add_experience {
        timestamps.insert(user_id, Instant::now());
        data.database
            .update_user_experience(
                user_id as i64,
                data.config.experience.experience_per_message as i64,
            )
            .await?;
    }

    Ok(())
}

pub async fn experience_voice_handler(
    data: &Data,
    old: &serenity::VoiceState,
    new: &serenity::VoiceState,
) -> Result<(), Error> {
    Ok(())
}

pub async fn experience_voice_updater() -> Result<(), Error> {
    Ok(())
}
