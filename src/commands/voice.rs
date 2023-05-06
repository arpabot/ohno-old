use crate::{queue, Context, Error};
use poise::serenity_prelude::model::channel::Channel;

/// ボイスチャンネルに接続します
#[poise::command(prefix_command, slash_command)]
pub async fn connect(ctx: Context<'_>) -> Result<(), Error> {
  if ctx.guild().is_none() {
    return Ok(());
  }
  let channel_id = ctx
    .guild()
    .unwrap()
    .voice_states
    .get(&ctx.author().id)
    .and_then(|voice_state| voice_state.channel_id);

  let connect_to = match channel_id {
    Some(channel) => channel,
    None => {
      ctx
        .say("ボイスチャンネルに接続しているか確認してください")
        .await?;
      return Ok(());
    }
  }
  .to_channel(&ctx.serenity_context().http)
  .await?;

  match connect_to {
    Channel::Guild(channel) => {
      ctx.defer().await?;
      let queue = queue::Queue::new(ctx, channel).await;
      match queue {
        Ok(q) => {
          let _ = &mut ctx
            .data()
            .queues
            .lock()
            .await
            .insert(ctx.guild_id().unwrap().into(), q);
          ctx.say("接続しました").await?;
        }
        Err(e) => {
          ctx.say(format!("エラーが発生しました```\n{:#?}```", e)).await?;
        }
      }
    }
    _ => {
      ctx
        .say("エラーが発生しました")
        .await?;
    }
  }
  Ok(())
}

/// ボイスチャンネルから退出します
#[poise::command(prefix_command, slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
  if ctx.guild().is_none() {
    return Ok(());
  }
  let is_connected = match ctx
    .guild()
    .unwrap()
    .voice_states
    .get(&ctx.author().id)
    .and_then(|voice_state| voice_state.channel_id)
  {
    Some(cid) => {
      let mut result = false;
      if let Some(q) = ctx
        .data()
        .queues
        .lock()
        .await
        .get(ctx.guild_id().unwrap().as_u64())
      {
        if let Some(bot_cid) = q.handler.lock().await.current_channel() {
          if *cid.as_u64() == bot_cid.0 {
            result = true;
          }
        }
      }
      result
    }
    None => false,
  };
  if ctx
    .data()
    .queues
    .lock()
    .await
    .contains_key(ctx.guild_id().unwrap().as_u64())
    && is_connected
  {
    let mut lock = ctx.data().queues.lock().await;
    let queue = lock.get(ctx.guild_id().unwrap().as_u64()).unwrap();
    let mut handler = queue.handler.lock().await;
    handler.leave().await?;
    handler.queue().stop();
    drop(handler);
    lock.remove(ctx.guild_id().unwrap().as_u64());
    ctx.say("退出しました。").await?;
  } else {
    ctx
      .say("あなたかBotがボイスチャンネルにいないか、あなたがBotと同じボイスチャンネルにいません。")
      .await?;
  }
  Ok(())
}
