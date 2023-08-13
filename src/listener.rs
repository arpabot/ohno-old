use crate::{Data, Error};
use poise::serenity_prelude::model::{
  gateway::Activity,
  id::{ChannelId, GuildId},
  user::OnlineStatus,
};
use poise::serenity_prelude::{self as serenity, Context};
use regex::Regex;

pub async fn event_listener(
  ctx: &serenity::Context,
  event: &poise::Event<'_>,
  _framework: poise::FrameworkContext<'_, Data, Error>,
  user_data: &Data,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
      let url = Regex::new(r#"https?://[\w/:%#$&?()~.=+-]+"#)?;
      let code_block = Regex::new(r#"(?sm)```.*```"#)?;
      let key: u64 = msg.guild_id.unwrap_or(GuildId(0)).into();
      #[allow(clippy::collapsible_if)]
      if user_data.queues.lock().await.contains_key(&key) {
        if user_data.queues.lock().await.get(&key).unwrap().channel_id
          == msg.channel(&ctx.http).await?.id()
          || user_data
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
            == msg.channel(&ctx.http).await?.id().into()
        {
          println!(
            "{:?}",
            serenity::content_safe(
              ctx,
              code_block
                .replace_all(
                  &url.replace_all(&msg.content, "URL省略"),
                  "コードブロック省略",
                )
                .chars()
                .take(150)
                .collect::<String>(),
              &serenity::utils::ContentSafeOptions::default(),
              &msg.mentions,
            )
          );
          user_data
            .queues
            .lock()
            .await
            .get(&key)
            .unwrap()
            .play(serenity::content_safe(
              ctx,
              code_block
                .replace_all(
                  &url.replace_all(&msg.content, "URL省略"),
                  "コードブロック省略",
                )
                .chars()
                .take(150)
                .collect::<String>(),
              &serenity::utils::ContentSafeOptions::default(),
              &msg.mentions,
            ))
            .await;
        }
      }
    }

    poise::Event::VoiceStateUpdate { old, new } => {
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
          .count()
          == 0
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
          user_data.queues.lock().await.remove(&key);
          text
            .say(
              &ctx.http,
              "ボイスチャンネルに誰もいなくなったため退出しました",
            )
            .await?;
        } else {
          if let Some(o) = old {
            if let Some(member) = &o.member {
              if new.channel_id.is_none() {
                let name = &member.nick.as_ref().unwrap_or(&member.user.name);
                play(user_data, key, ctx, &format!("{name}が退出しました")).await;
              }
            }
          } else {
            if let Some(member) = &new.member {
              let name = &member.nick.as_ref().unwrap_or(&member.user.name);
              play(user_data, key, ctx, &format!("{name}が入室しました")).await;
            }
          }
        }
      }
    }
    _ => (),
  }

  Ok(())
}

async fn play(user_data: &Data, key: u64, ctx: &Context, text: &str) {
  user_data
    .queues
    .lock()
    .await
    .get(&key)
    .unwrap()
    .play(serenity::content_safe(
      ctx,
      text,
      &serenity::utils::ContentSafeOptions::default(),
      &vec![],
    ))
    .await;
}
