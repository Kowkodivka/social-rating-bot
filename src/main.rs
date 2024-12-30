use std::sync::Arc;

use components::{config::Config, database::Database};
use handlers::experience::ExperienceHandler;
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use serenity::all::{ClientBuilder, GatewayIntents};
use tokio::sync::Mutex;

mod commands;
mod components;
mod handlers;

struct Data {
    config: Config,
    database: Arc<Database>,
    translations: Arc<components::translation::Translations>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().compact().init();

    // Загрузка конфигурации
    let config = Config::load("./Config.toml")?;

    // Клонируем config для использования в setup и ClientBuilder
    let config_for_setup = config.clone();

    // Инициализация базы данных
    let database = Arc::new(Database::new("sqlite://store.db").await?);
    database.initialize().await?;

    // Загрузка переводов
    let translations = Arc::new(components::translation::read_ftl()?);

    // Регистрация команд
    let mut commands = vec![
        commands::basic::ping(),
        commands::experience::experience(),
        commands::reputation::repute(),
        commands::reputation::diminish(),
        commands::reputation::show_message_reputation(),
        commands::reputation::show_user_reputation(),
    ];
    components::translation::apply_translations(&translations, &mut commands);

    // Подготовка данных для передачи в контекст
    let shared_data = Arc::new(Mutex::new(Data {
        config: config.clone(),
        database: Arc::clone(&database),
        translations: Arc::clone(&translations),
    }));

    // Настройка фреймворка
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let framework = Framework::builder()
        .options(FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(config.discord.prefix.clone()),
                ..Default::default()
            },
            commands,
            ..Default::default()
        })
        .setup(move |_ctx, _ready, _framework| {
            let data = Data {
                config: config_for_setup,
                database: Arc::clone(&database),
                translations: Arc::clone(&translations),
            };
            Box::pin(async move { Ok(data) })
        })
        .build();

    // Создание и запуск клиента
    let mut client = ClientBuilder::new(&config.discord.token, intents)
        .framework(framework)
        .event_handler(ExperienceHandler {
            data: Arc::clone(&shared_data),
        })
        .await?;

    client.start().await?;

    Ok(())
}
