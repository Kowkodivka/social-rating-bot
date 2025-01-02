use components::{config::Config, database::Database};
use poise::serenity_prelude as serenity;
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

mod commands;
mod components;
mod handlers;

struct Data {
    config: Arc<Config>,
    database: Arc<Database>,
    translations: Arc<components::translation::Translations>,
    timestamps: Arc<Mutex<HashMap<u64, Instant>>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().compact().init();

    let config = Arc::new(Config::load("./Config.toml")?);

    let database = Arc::new(Database::new("sqlite://store.db").await?);
    database.initialize().await?;

    let translations = Arc::new(components::translation::read_ftl()?);

    let timestamps = Arc::new(Mutex::new(HashMap::new()));

    let mut commands = vec![
        // Basic
        commands::basic::ping(),
        // Experience
        commands::experience::experience(),
        // Reputation
        commands::reputation::repute(),
        commands::reputation::reverse_repute(),
        commands::reputation::show_message_reputation(),
        commands::reputation::show_user_reputation(),
    ];

    components::translation::apply_translations(&translations, &mut commands);

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = Framework::builder()
        .options(FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(config.discord.prefix.clone()),
                ..Default::default()
            },
            event_handler: |_ctx, event, _framework, data| {
                Box::pin(async move {
                    match event {
                        serenity::FullEvent::Message { new_message } => {
                            handlers::experience::experience_message_handler(data, new_message)
                                .await?;
                        }

                        serenity::FullEvent::VoiceStateUpdate { old, new } => {
                            if let Some(old) = old {
                                handlers::experience::experience_voice_handler(data, old, new)
                                    .await?;
                            }
                        }
                        _ => {}
                    }
                    Ok(())
                })
            },
            commands,
            ..Default::default()
        })
        .setup({
            let config = Arc::clone(&config);
            let database = Arc::clone(&database);
            let translations = Arc::clone(&translations);
            let timestamps = Arc::clone(&timestamps);
            move |ctx, _ready, framework| {
                Box::pin(async move {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data {
                        config,
                        database,
                        translations,
                        timestamps,
                    })
                })
            }
        })
        .build();

    let mut client = serenity::ClientBuilder::new(&config.discord.token, intents)
        .framework(framework)
        .await?;

    tokio::spawn(handlers::experience::experience_voice_updater());

    client.start().await?;
    Ok(())
}
