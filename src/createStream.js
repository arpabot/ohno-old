const sdk = require("microsoft-cognitiveservices-speech-sdk");
const { createReadStream, unlinkSync } = require("fs");

module.exports = (text, apiKey, region, rate = "1.2") => {
  return new Promise(async (resolve, reject) => {
    const fname = Math.random().toString(16).split(".").pop() + ".ogg";
    const audioConfig = sdk.AudioConfig.fromAudioFileOutput(fname);
    const speechConfig = sdk.SpeechConfig.fromSubscription(apiKey, region);
    speechConfig.speechSynthesisOutputFormat = 23;
    let synthesizer = new sdk.SpeechSynthesizer(speechConfig, audioConfig);
    let ssml = `<speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="ja-JP">
    <voice name="ja-JP-NanamiNeural">
        <prosody rate="${rate}">
            ${sdk.SpeechSynthesizer.XMLEncode(text)}
        </prosody>
      </voice>
    </speak>`;

    synthesizer.speakSsmlAsync(
      ssml,
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
