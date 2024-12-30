use crate::Data;
use serenity::{
    all::{Context as SerenityContext, EventHandler, Message},
    async_trait,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::error;

pub struct ExperienceHandler {
    pub data: Arc<Mutex<Data>>,
}

#[async_trait]
impl EventHandler for ExperienceHandler {
    async fn message(&self, _ctx: SerenityContext, msg: Message) {
        if msg.author.bot {
            return;
        }

        let experience_gain = 10;
        let user_id = msg.author.id.get();

        let data = self.data.lock().await;
        if let Err(e) = data
            .database
            .update_user_experience(user_id as i64, experience_gain)
            .await
        {
            error!("Failed to update experience: {:?}", e);
        }
    }
}
