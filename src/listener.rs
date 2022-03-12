use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::model::{gateway::Activity, user::OnlineStatus};
use regex::Regex;

pub async fn event_listener(
  ctx: &serenity::Context,
  event: &poise::Event<'_>,
  _framework: &poise::Framework<Data, Error>,
  user_data: &Data,
) -> Result<(), Error> {
  match event {
    poise::Event::Ready { data_about_bot: _ } => {
      println!("Ready!");
      ctx.set_presence(Some(Activity::listening("Your Voice 24/7")), OnlineStatus::DoNotDisturb).await;
    }
    poise::Event::Message { new_message: msg } => {
      let re = Regex::new(r#"https?://[\w/:%#$&?()~.=+-]+"#)?;
      let key: u64 = msg.guild_id.unwrap().into();
  
        if user_data.queues.lock().await.contains_key(&key) && (user_data.queues.lock().await.get(&key).unwrap().channel_id
          == msg.channel(&ctx.http).await?.id())
        {
          user_data
            .queues
            .lock()
            .await
            .get(&key)
            .unwrap()
            .play(re.replace_all(&msg.content_safe(&ctx.cache), "URL省略").chars().take(500).collect::<String>())
            .await;
      }
    }
    _ => (),
  }

  Ok(())
}
