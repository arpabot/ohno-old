use crate::Context;
use audiopus::{coder::Decoder, Channels, SampleRate};
use cognitive_services_speech_sdk_rs::{
  audio::{AudioConfig, PullAudioOutputStream},
  common::OutputFormat,
  speech::{SpeechConfig, SpeechSynthesizer},
};
use poise::serenity_prelude::{
  async_trait,
  id::{ChannelId, GuildId},
  model::channel::GuildChannel,
  Mutex as PoiseMutex,
};
use songbird::{
  events::{Event, TrackEvent},
  input::{codec::Codec, reader::Reader, Input},
  EventContext, EventHandler,
};
use std::{
  env,
  sync::{Arc, Mutex},
};

pub struct Queue {
  pub strings: Mutex<Vec<String>>,
  pub playing: Mutex<bool>,
  pub guild_id: GuildId,
  pub channel_id: ChannelId,
  pub handler: Arc<PoiseMutex<songbird::Call>>,
}

struct OnEnd {
  queue: &'static Queue,
}

impl Queue {
  pub async fn play(&'static self, text: Option<String>) {
    if *self.playing.lock().unwrap() {
      let mut strings = self.strings.lock().unwrap();
      if let Some(s) = text {
        strings.push(s);
      }
    } else {
      *self.playing.lock().unwrap() = true;
      let s = match text {
        Some(s) => s,
        _ => {
          let mut strings = self.strings.lock().unwrap();
          if strings.len() != 0 {
            strings.remove(0)
          } else {
            "".into()
          }
        }
      };
      let stream = PullAudioOutputStream::create_pull_stream().unwrap();
      let audio_config = AudioConfig::from_stream_output(&stream).unwrap();
      let mut speech_config =
        SpeechConfig::from_subscription(env::var("apiKey").unwrap(), env::var("region").unwrap())
          .unwrap();
      speech_config
        .set_get_output_format(OutputFormat::Detailed)
        .unwrap();
      speech_config
        .set_get_speech_synthesis_output_format("Ogg48Khz16BitMonoOpus".into())
        .unwrap();
      let speech_synthesizer = SpeechSynthesizer::from_config(speech_config, audio_config).unwrap();
      let speech_result = speech_synthesizer.speak_ssml_async(&format!("<speak version=\"1.0\" xmlns=\"http://www.w3.org/2001/10/synthesis\" xml:lang=\"ja-JP\">\
      <voice name=\"ja-JP-NanamiNeural\">\
          <prosody rate=\"{}\">\
              {}\
          </prosody>\
        </voice>\
      </speak>", "1.2", s)).await;
      if let Ok(bytes) = speech_result {
        let reader = Reader::from(bytes.audio_data);
        let decoder = Decoder::new(SampleRate::Hz48000, Channels::Mono).unwrap();
        let kind = Codec::Opus(songbird::input::codec::OpusDecoderState::from_decoder(
          decoder,
        ));
        let input = Input::new(false, reader, kind, songbird::input::Container::Raw, None);
        self
          .handler
          .lock()
          .await
          .play_source(input)
          .add_event(Event::Track(TrackEvent::End), OnEnd { queue: self })
          .ok();
      }
    }
  }

  pub async fn new(ctx: Context<'_>, vc: GuildChannel) -> Result<Self, ()> {
    let guild_id = ctx.guild_id().unwrap();
    let channel_id = ctx.channel_id();
    let manager = songbird::get(ctx.discord())
      .await
      .expect("Songbird Voice client placed in at initialisation.")
      .clone();
    let (handler, result) = manager.join(guild_id, vc.id).await;
    match result {
      Ok(()) => Ok(Self {
        strings: Mutex::new(vec![]),
        playing: Mutex::new(false),
        channel_id,
        guild_id,
        handler,
      }),
      Err(e) => {
        ctx.say(format!("Error: {}", e)).await.ok();
        Err(())
      }
    }
  }
}

#[async_trait]
impl EventHandler for OnEnd {
  async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
    *self.queue.playing.lock().unwrap() = false;
    self.queue.play(None).await;
    None
  }
}
