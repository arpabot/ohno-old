use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{
  model::{gateway::Activity, id::{ChannelId, GuildId}, user::OnlineStatus},
};
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
      ctx
        .set_presence(
          Some(Activity::listening("Your Voice 24/7")),
          OnlineStatus::DoNotDisturb,
        )
        .await;
    }
    poise::Event::Message { new_message: msg } => {
      let re = Regex::new(r#"https?://[\w/:%#$&?()~.=+-]+"#)?;
      let key: u64 = msg.guild_id.unwrap_or(GuildId(0)).into();
      #[allow(clippy::collapsible_if)]
      if user_data.queues.lock().await.contains_key(&key) {
        if user_data.queues.lock().await.get(&key).unwrap().channel_id
          == msg.channel(&ctx.http).await?.id()
        {
          user_data
            .queues
            .lock()
            .await
            .get(&key)
            .unwrap()
            .play(
              re.replace_all(&msg.content_safe(&ctx.cache), "URL省略")
                .chars()
                .take(500)
                .collect::<String>(),
            )
            .await;
        }
      }
    }
    poise::Event::VoiceStateUpdate { old: _, new } => {
      if let Some(gid) = new.guild_id {
        let key: u64 = gid.into();
        if !user_data.queues.lock().await.contains_key(&key) {
          return Ok(());
        }
        if user_data
          .queues
          .lock()
          .await
          .get(&key)
          .unwrap()
          .handler
          .lock()
          .await
          .current_channel()
          .is_none()
        {
          return Ok(());
        }
        let channel_id: ChannelId = user_data
          .queues
          .lock()
          .await
          .get(&key)
          .unwrap()
          .handler
          .lock()
          .await
          .current_channel()
          .unwrap()
          .0
          .into();
        if channel_id.to_channel(&ctx.http).await?.guild().is_none() {
          return Ok(());
        }
        let channel = channel_id.to_channel(&ctx.http).await?.guild().unwrap();
        let members = channel.members(&ctx.cache).await?;
        if members
          .iter()
          .filter(|x| x.user.id.as_u64() != ctx.cache.current_user_id().as_u64())
          .count() == 0
        {
          user_data
            .queues
            .lock()
            .await
            .get(&key)
            .unwrap()
            .handler
            .lock()
            .await
            .leave()
            .await?;
          let text = user_data
            .queues
            .lock()
            .await
            .get(&key)
            .unwrap()
            .channel_id
            .to_channel(&ctx.http)
            .await?
            .guild()
            .unwrap();
          text
            .say(
              &ctx.http,
              "ボイスチャンネルに誰もいなくなったため退出しました",
            )
            .await?;
          user_data.queues.lock().await.remove(&key);
        }
      }
    }
    _ => (),
  }

  Ok(())
}
