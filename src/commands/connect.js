const {
  joinVoiceChannel,
  createAudioPlayer,
  createAudioResource,
  StreamType,
  VoiceConnectionDisconnectReason,
} = require("@discordjs/voice");
const createStream = require("../createStreamWatson");

module.exports = async (message) => {
  if (!message.member.voice.channel)
    return message.reply(
      "あなたがまずボイスチャンネルに参加する必要があります。"
    );
  if (!message.member.voice.channel.joinable)
    return message.reply(
      "あなたがいるボイスチャンネルに参加することができません。"
    );
  let cache;
  if (
    await dicts
      .queryPromise("show tables like ?", [message.guild.id])
      .catch(console.error)
  )
    cache = (
      await dicts
        .queryPromise(`select * from \`${message.guild.id}\``)
        .catch(console.error)
    )?.[0];
  const serverQueue = {};
  serverQueue.connection = joinVoiceChannel({
    adapterCreator: message.guild.voiceAdapterCreator,
    guildId: message.guild.id,
    channelId: message.member.voice.channel.id,
    selfDeaf: true,
    selfMute: false,
  });
  serverQueue.voiceChannel = message.member.voice.channel;
  serverQueue.textChannel = message.channel;
  serverQueue.player = createAudioPlayer();
  serverQueue.isPlaying = false;
  serverQueue.texts = [];
  serverQueue.play = async (text) => {
    serverQueue.isPlaying = true;
    const stream = await createStream(
      text,
      process.env.apiKey,
      process.env.region
    ).catch(console.error);
    if (!stream) return serverQueue.textChannel.send("再生に失敗しました");
    const resource = createAudioResource(stream, {
      inputType: StreamType.OggOpus,
    });
    resource.playStream.on("end", () => {
      if (serverQueue.texts[0])
        return serverQueue.play(serverQueue.texts.shift());
      serverQueue.isPlaying = false;
    });
    serverQueue.player.play(resource);
  };
  serverQueue.connection.subscribe(serverQueue.player);
  queues.set(message.channel.id, serverQueue);
  dictCache.set(message.guild.id, cache);
  message.reply("接続しました。");
};
