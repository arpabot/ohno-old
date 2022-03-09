use crate::{Data, Error};
use poise::serenity_prelude as serenity;

pub async fn event_listener(
  _ctx: &serenity::Context,
  event: &poise::Event<'_>,
  _framework: &poise::Framework<Data, Error>,
  _user_data: &Data,
) -> Result<(), Error> {
  match event {
    poise::Event::Ready { data_about_bot: _ } => {
      println!("Ready!")
    }
    _ => {}
  }

  Ok(())
}
