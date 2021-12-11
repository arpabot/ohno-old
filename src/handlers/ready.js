module.exports = async (client) => {
  setInterval(() => {
    client.user.setActivity(process.env.prefix + "s | " + queues.size + "VC");
  }, 3000);
  console.log("ready!");
};
