const fetch = require("node-fetch");

module.exports = (text, apiKey, endpoint, format = "audio/ogg;codecs=opus") => {
  return new Promise(async (resolve, reject) => {
    resolve(
      await fetch(endpoint + "/v1/synthesize?voice=ja-JP_EmiV3Voice", {
        method: "POST",
        headers: {
          Authorization:
            "Basic " + Buffer.from("apikey:" + apiKey).toString("base64"),
          "Content-Type": "application/json",
          Accept: format,
        },
        body: JSON.stringify({
          text,
        }),
      })
        .then((r) => r.body)
        .catch(console.error)
    );
  });
};
