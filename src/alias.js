const d = require("./commands/alias.json");
const { writeFileSync } = require("fs");
const { join } = require("path");
for (let command in d) {
  for (let alias of d[command]) {
    writeFileSync(
      join(__dirname, "commands", "_" + alias + ".js"),
      'module.exports = require("./' + command + '.js");\n'
    );
  }
}
