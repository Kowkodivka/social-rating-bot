use std::sync::Arc;

use components::database::Database;
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use serenity::all::{ClientBuilder, GatewayIntents};
use services::experience::ExperienceHandler;
use tokio::sync::Mutex;

mod commands;
mod components;
mod services;

struct Data {
    database: Arc<Database>,                                  // Оборачиваем в Arc
    translations: Arc<components::translation::Translations>, // Оборачиваем в Arc
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().compact().pretty().init();

    let config = components::config::Config::load().expect("Invalid .env configuration");

    let mut commands = vec![commands::ping(), commands::experience()];

    let database = Arc::new(Database::new("sqlite://store.db").await?); // Используем Arc
    database.initialize().await?;

    let translations =
        Arc::new(components::translation::read_ftl().expect("Failed to read translation files")); // Используем Arc
    components::translation::apply_translations(&translations, &mut commands);

    // Создаем Data до setup
    let data = Arc::new(Mutex::new(Data {
        database: database.clone(),         // Передаем Arc
        translations: translations.clone(), // Передаем Arc
    }));

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let framework = Framework::builder()
        .options(FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(config.discord_prefix),
                ..Default::default()
            },
            commands: commands,
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            // Используем move
            let database = database.clone(); // Клонируем Arc
            let translations = translations.clone(); // Клонируем Arc

            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    database,
                    translations,
                })
            })
        })
        .build();

    let mut client = ClientBuilder::new(&config.discord_token, intents)
        .framework(framework)
        .event_handler(ExperienceHandler { data: data.clone() }) // Передаем данные обработчику
        .await?;

    client.start().await?;

    Ok(())
}
