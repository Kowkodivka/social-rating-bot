use serenity::{
    all::{Context as SerenityContext, EventHandler, Message},
    async_trait,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::Data;

pub struct ExperienceHandler {
    pub data: Arc<Mutex<Data>>, // Ссылка на данные
}

#[async_trait]
impl EventHandler for ExperienceHandler {
    async fn message(&self, _ctx: SerenityContext, msg: Message) {
        if msg.author.bot {
            return;
        }

        let experience_gain: i64 = 10;
        let user_id = msg.author.id.get();

        let data = self.data.lock().await; // Получаем доступ к данным
        let result = data
            .database
            .update_user_experience(user_id.try_into().unwrap(), experience_gain)
            .await;

        match result {
            Ok(_) => info!("Experience updated for user {}", user_id),
            Err(e) => error!("Failed to update experience: {:?}", e),
        }
    }
}
