use crate::{Data, Error};
use poise::serenity_prelude as serenity;

pub async fn event_listener<'a: 'static>(
  _ctx: &serenity::Context,
  event: &poise::Event<'a>,
  _framework: &poise::Framework<Data, Error>,
  user_data: &'a Data,
) -> Result<(), Error> {
  match event {
    poise::Event::Ready { data_about_bot: _ } => {
      println!("Ready!")
    }
    poise::Event::Message { new_message: msg } => {
      let key: u64 = msg.guild_id.unwrap().into();
      if let Some(queue) = user_data.queues.lock().unwrap().get(&key) {
        queue.play(msg.content.clone().into());
      };
    }
    _ => (),
  }

  Ok(())
}
