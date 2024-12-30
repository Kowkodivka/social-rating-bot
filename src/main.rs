use std::sync::Arc;

use components::database::Database;
use handlers::experience::ExperienceHandler;
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use serenity::all::{ClientBuilder, GatewayIntents};
use tokio::sync::Mutex;

mod commands;
mod components;
mod handlers;

struct Data {
    database: Arc<Database>,
    translations: Arc<components::translation::Translations>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().compact().init();

    let config = components::config::Config::load()?;

    let mut commands = vec![
        // Basic
        commands::basic::ping(),
        // Experience
        commands::experience::experience(),
        // Reputation
        commands::reputation::repute(),
        commands::reputation::diminish(),
        commands::reputation::show_message_reputation(),
        commands::reputation::show_user_reputation(),
    ];

    let database = Arc::new(Database::new("sqlite://store.db").await?);
    database.initialize().await?;

    let translations = Arc::new(components::translation::read_ftl()?);
    components::translation::apply_translations(&translations, &mut commands);

    let data = Arc::new(Mutex::new(Data {
        database: database.clone(),
        translations: translations.clone(),
    }));

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let framework = Framework::builder()
        .options(FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(config.discord_prefix),
                ..Default::default()
            },
            commands,
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            let database = database.clone();
            let translations = translations.clone();
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
        .event_handler(ExperienceHandler { data })
        .await?;

    client.start().await?;

    Ok(())
}
