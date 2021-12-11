module.exports = async (message) => {
  if (!message.member.voice.channel)
    return message.reply("あなたはボイスチャンネルにいません。");
  const serverQueue = queues.find(
    (x) => x.voiceChannel.id === message.member.voice.channel.id
  );
  if (!serverQueue)
    return message.reply("あなたはボットがいるボイスチャンネルにいません。");
  serverQueue.term = true;
  message.guild.me.voice.disconnect();
  message.reply("切断しました。");
  queues.delete(serverQueue.textChannel.id);
};
