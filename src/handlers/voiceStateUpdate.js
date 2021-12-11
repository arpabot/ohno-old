module.exports = async (oldState, newState) => {
  const serverQueue = queues.find(
    (x) => x.voiceChannel.id === (oldState || newState)?.channel?.id
  );

  if (!serverQueue) return;
  if (!newState.channel) {
    if (serverQueue.term) return;
    serverQueue.term = true;
    await newState.guild.me.voice.disconnect();
    await serverQueue.textChannel
      .send("ボイスチャンネルに誰もいなくなったため退出しました。")
      .catch(console.error);
    queues.delete(serverQueue.textChannel.id);
  }
};
