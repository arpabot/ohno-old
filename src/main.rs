use dotenv::dotenv;
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use songbird::SerenityInit;
use std::{collections::HashMap, env, sync::Mutex};
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
  Framework::build()
    .token(token)
    .user_data_setup(move |_ctx, _ready, _framework| {
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
        commands::voice::connect(),
      ],
      prefix_options: PrefixFrameworkOptions {
        prefix: env::var("prefix").ok(),
        ..Default::default()
      },
      listener: |ctx, event, framework, user_data| {
        Box::pin(listener::event_listener(ctx, event, framework, user_data))
      },
      ..Default::default()
    })
    .client_settings(|c| c.register_songbird())
    .run()
    .await
    .unwrap();
}
