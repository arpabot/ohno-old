const { promisify } = require("util");

module.exports = async (client) => {
  dicts.connect();
  dicts.queryPromise = promisify(dicts.query);
  setInterval(() => {
    client.user.setActivity(process.env.prefix + "s | " + queues.size + "VC");
  }, 3000);
  console.log("ready!");
};
