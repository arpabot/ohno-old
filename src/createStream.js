const sdk = require("microsoft-cognitiveservices-speech-sdk");
const { createReadStream, unlinkSync } = require("fs");
sdk.PropertyId.SpeechServiceConnection_SynthOutputFormat =
  "ogg-48khz-16bit-mono-opus";

module.exports = (text, apiKey, region) => {
  return new Promise(async (resolve, reject) => {
    const fname = Math.random().toString(16).split(".").pop() + ".ogg";
    const audioConfig = sdk.AudioConfig.fromAudioFileOutput(fname);
    //audioConfig.format = "OGG_OPUS";
    const speechConfig = sdk.SpeechConfig.fromSubscription(apiKey, region);
    speechConfig.speechSynthesisOutputFormat = 23;
    speechConfig.speechSynthesisLanguage = "ja-JP";
    speechConfig.speechSynthesisVoiceName = "ja-JP-NanamiNeural";
    let synthesizer = new sdk.SpeechSynthesizer(speechConfig, audioConfig);
    //synthesizer.audioOutputFormat = "OGG_OPUS";

    synthesizer.speakTextAsync(
      text,
      function (result) {
        if (result.reason === sdk.ResultReason.SynthesizingAudioCompleted) {
          const stream = createReadStream(fname);
          stream.on("end", () => unlinkSync(fname));
          synthesizer.close();
          resolve(stream);
        } else {
          unlinkSync(fname);
          synthesizer.close();
          reject(
            "Speech synthesis canceled, " +
              result.errorDetails +
              "\nDid you update the subscription info?"
          );
        }
      },
      (e) => {
        unlinkSync(fname);
        synthesizer.close();
        reject(e);
      }
    );
  });
};
