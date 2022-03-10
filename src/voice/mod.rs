use reqwest::Client;
use bytes::Bytes;
use std::fmt::{Display, Formatter};
use serde_json::Value;

pub struct VoiceClient {
  key: String,
  region: String,
  client: Client,
  voice: String,
  rate: f64,
  output: OutputKind
}

type Result<T, E = Error> = std::result::Result<T, E>;

impl VoiceClient {
  pub fn new<T: AsRef<str>>(key: T, region: T, output: OutputKind) -> Self{
    Self {
      key: key.as_ref().to_string(),
      region: region.as_ref().to_string(),
      client: Client::new(),
      voice: String::from("ja-JP-NanamiNeural"),
      rate: 1.2,
      output
    }
  }
  async fn authorize(&self) -> Result<String> {
    match self.client.post(self.authurl())
    .header("Ocp-Apim-Subscription-Key", &*self.key.clone())
    .header("Content-Length", 0)
    .send().await {
      Ok(resp) => {
        match resp.text().await {
          Ok(token) => Ok(token),
          Err(e) => Err(Error::ReqwestError(e))
        }
      }
      Err(e) => Err(Error::ReqwestError(e))
    }
  }
  pub async fn speech<T: AsRef<str>>(&self, text: T) -> Result<Bytes> {
      let fmt_ssml = format!(
      "<speak version=\"1.0\" xmlns=\"http://www.w3.org/2001/10/synthesis\" xml:lang=\"ja-JP\">\
        <voice name=\"{voice}\">\
            <prosody rate=\"{rate}\">\
              {text}\
            </prosody>\
          </voice>\
        </speak>", rate=self.rate, text=text.as_ref(), voice=self.voice);
      match self.client.post(self.baseurl("v1"))
      .header("Content-Type", "application/ssml+xml")
      .header("User-Agent", "OHNO/0.1.0")
      .header("Authorization", &*format!("Bearer {token}", token=self.authorize().await.unwrap()))
      .header("X-Microsoft-OutputFormat", &*self.output.to_string())
      .body(fmt_ssml)
      .send().await {
        Ok(resp) => {
          match resp.bytes().await {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(Error::ReqwestError(e))
          }
        },
        Err(e) => Err(Error::ReqwestError(e))
      }
  }
  pub fn current_voice(&self) -> String {
    self.voice.clone()
  }
  fn authurl(&self) -> String {
    format!("https://{region}.api.cognitive.microsoft.com/sts/v1.0/issueToken", region=self.region)
  }
  fn baseurl(&self, endpoint: &str) -> String {
    format!("https://{region}.tts.speech.microsoft.com/cognitiveservices/{ep}", ep=endpoint, region=self.region)
  }
  pub async fn set_voice(&mut self, voice: &str) -> Result<()>{
    match self.client.post(self.baseurl("voices/list"))
    .header("Authorization", &*format!("Bearer {token}", token=self.authorize().await.unwrap()))
    .header("User-Agent", "OHNO/0.1.0")
    .send().await {
      Ok(res) => {
        match res.text().await {
          Ok(text) => {
             match serde_json::from_str::<Value>(&text) {
              Ok(json) => {
                match json.get("voices") {
                  Some(voicevalue) => {
                    match voicevalue {
                      Value::Array(voices) => {
                        let mut err = Err(Error::VoiceNotFound);
                        for voi in voices {
                          if let Some(vo) = voi.get("ShortName") {
                            match vo {
                              Value::String(vv) => {
                                if vv == voice {
                                  self.voice = vv.clone();
                                  err = Ok(());
                                  break
                                }
                              },
                              _ => continue
                            }
                          }
                        }
                        err
                      },
                      _ => Err(Error::VoiceNotFound)
                    }
                  },
                  None => Err(Error::VoiceNotFound)
                }
              },
              Err(e) => Err(Error::SerdeError(e))
            }
          },
          Err(e) => Err(Error::ReqwestError(e))
        }
      },
      Err(e) => Err(Error::ReqwestError(e))
    }
  }
}

pub enum OutputKind {
  Raw48KHz16BitMonoPcm
}

impl ToString for OutputKind {
  fn to_string(&self) -> String {
    match self {
      Raw48KHz16BitMonoPcm => "raw-48khz-16bit-mono-pcm".to_string()
    }
  }
}

#[derive(Debug)]
pub enum Error {
  ReqwestError(reqwest::Error),
  VoiceNotSet,
  VoiceNotFound,
  SerdeError(serde_json::Error)
}
impl std::error::Error for Error {}

impl Display for Error{
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error>{
    match self {
      Self::ReqwestError(e) => e.fmt(fmt),
      Self::SerdeError(e) => e.fmt(fmt),
      Self::VoiceNotFound => fmt.write_str("VoiceNotFound"),
      Self::VoiceNotSet => fmt.write_str("VoiceNotSet")
    }
  }
}