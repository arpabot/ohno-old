use crate::voice::{OutputKind, VoiceClient};
use crate::{Context, Error};
use poise::serenity_prelude::{
  model::{channel::GuildChannel, id::ChannelId},
  Mutex as PoiseMutex,
};
use songbird::{tracks::create_player,input::{codec::Codec, reader::Reader, Input}};
use std::{env, sync::Arc};

#[derive(Debug)]
pub struct Queue {
  pub channel_id: ChannelId,
  pub handler: Arc<PoiseMutex<songbird::Call>>,
}

impl Queue {
  pub async fn play(&self, text: String) {
    let voice_client = VoiceClient::new(
      env::var("apiKey").unwrap(),
      env::var("region").unwrap(),
      OutputKind::Raw48KHz16BitMonoPcm,
    );
    let speech_result = voice_client.speech(text).await;
    if let Ok(bytes) = speech_result {
      let reader = Reader::from(bytes.to_vec());
      let kind = Codec::Pcm;
      let input = Input::new(false, reader, kind, songbird::input::Container::Raw, None);
      let (mut audio, _) = create_player(input);
      audio.set_volume(0.5);
      self.handler.lock().await.enqueue(audio);
    }
  }

  pub async fn new(ctx: Context<'_>, vc: GuildChannel) -> Result<Self, Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.discord())
      .await
      .expect("Songbird Voice client placed in at initialisation.")
      .clone();
    let (handler, result) = manager.join(guild_id, vc.id).await;
    let mut lock = handler.lock().await;
    lock.deafen(true).await?;
    drop(lock);
    result?;
    Ok(Self {
      channel_id: ctx.channel_id(),
      handler,
    })
  }
}
