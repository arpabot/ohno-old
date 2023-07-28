use dotenv::dotenv;
use futures::lock::Mutex;
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions, serenity_prelude::GatewayIntents};
use songbird::SerenityInit;
use std::{collections::HashMap, env};
mod voice;

mod commands;
mod listener;

pub mod queue;
pub struct Data {
  pub queues: Mutex<HashMap<u64, queue::Queue>>,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
  dotenv().ok();
  let token = env::var("token").unwrap();
  Framework::builder()
  .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
    .token(token)
    .setup(move |_ctx, _ready, _framework| {
      Box::pin(async move {
        Ok(Data {
          queues: Mutex::new(HashMap::new()),
        })
      })
    })
    .options(FrameworkOptions {
      commands: vec![
        commands::general::help(),
        commands::general::register(),
        commands::voice::join(),
        commands::voice::leave(),
      ],
      prefix_options: PrefixFrameworkOptions {
        prefix: env::var("prefix").ok(),
        ..Default::default()
      },
      event_handler: |ctx, event, framework, data| {
        Box::pin(listener::event_listener(ctx, event, framework, data))
      },
      ..Default::default()
    })
    .client_settings(|c| c.register_songbird())
    .run()
    .await
    .unwrap();
}
