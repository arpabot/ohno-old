use crate::{Data, Error};
use poise::serenity_prelude as serenity;

pub async fn event_listener(
  _ctx: &serenity::Context,
  event: &poise::Event<'_>,
  _framework: &poise::Framework<Data, Error>,
  user_data: &Data,
) -> Result<(), Error> {
  match event {
    poise::Event::Ready { data_about_bot: _ } => {
      println!("Ready!")
    }
    poise::Event::Message { new_message: msg } => {
      let key: u64 = msg.guild_id.unwrap().into();
      user_data.queues.lock().unwrap().get(&key).unwrap().play(msg.content.clone()).await;
    }
    _ => (),
  }

  Ok(())
}
