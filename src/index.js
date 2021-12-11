const { Client } = require("discord.js");
const { Collection } = require("@discordjs/collection");
const { readdirSync } = require("fs");
const { join } = require("path");
require("dotenv").config();
const client = new Client({
  intents: 32509,
});
global.commands = {};
global.queues = new Collection();
global.sql = mysql.createConnection({
  host: process.env.dbHost, 
  user: process.env.dbUser, 
  password: process.env.dbPassword, 
  database: process.env.dbDatabase, 
  port: process.env.dbPort
});

readdirSync(join(__dirname, "handlers")).forEach((fname) => {
  const f = require(join(__dirname, "handlers", fname));
  client.on(fname.replace(".js", ""), (...i) =>
    i || i[0] ? f(...i) : f(client)
  );
});

readdirSync(join(__dirname, "commands")).forEach((fname) => {
  if (!fname.endsWith(".js")) return;
  global.commands[fname.replace(".js", "").replace("_", "")] = require(join(
    __dirname,
    "commands",
    fname
  ));
});

client.login(process.env.token);
