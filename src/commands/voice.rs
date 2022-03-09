use crate::{queue, Context, Error};
use poise::serenity_prelude::model::channel::Channel;

#[poise::command(prefix_command, slash_command)]
pub async fn connect(ctx: Context<'_>) -> Result<(), Error> {
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
  .to_channel(&ctx.discord().http)
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
            .unwrap()
            .insert(ctx.guild_id().unwrap().into(), q);
          ctx.say("接続しました").await?;
        }
        _ => {
          ctx.say("エラーが発生しました").await?;
        }
      }
    }
    _ => {
      ctx
        .say("ボイスチャンネルに接続しているか確認してください")
        .await?;
    }
  }
  Ok(())
}
