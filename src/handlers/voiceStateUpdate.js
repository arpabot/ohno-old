module.exports = async (oldState, newState) => {
  const serverQueue = queues.find(
    (x) => x.voiceChannel.id === (oldState || newState)?.channel?.id
  );

  if (!serverQueue) return;
  if (serverQueue.voiceChannel.members.size === 1) {
    if (serverQueue.term) return;
    serverQueue.term = true;
    await serverQueue.connection.disconnect();
    await serverQueue.textChannel
      .send("ボイスチャンネルに誰もいなくなったため退出しました。")
      .catch(console.error);
    queues.delete(serverQueue.textChannel.id);
  }
};
