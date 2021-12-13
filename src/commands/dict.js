const mysql = require("mysql");

module.exports = async (message, _args) => {
  const [subCommand, ...args] = _args;
  let word, yomi;
  switch (subCommand) {
    case "edit":
      [word, yomi] = args;
      if (word.length >= 100 || yomi.length >= 100)
        return message.reply(
          "読みまたは単語は100文字以内でなければなりません。"
        );
      let dict = (
        await dicts
          .queryPromise(`select * from \`${message.guild.id}\``)
          .catch(console.error)
      )?.[0];
      if (dict?.length >= 50)
        return message.reply("登録数は50個以下でなければなりません。");
      if (!dict)
        await dicts
          .queryPromise(
            `create table \`${message.guild.id}\` (is_premium boolean not null default false)`
          )
          .catch(console.error);
      const isAdded =
        (!dict?.[word]
          ? await dicts
              .queryPromise(
                `alter table \`${message.guild.id}\` add ${mysql
                  .escape(word)
                  .slice(1, -1)} varchar(255)`
              )
              .catch(console.error)
          : true) &&
        (await dicts
          .queryPromise(
            `update \`${message.guild.id}\` set ${mysql
              .escape(word)
              .slice(1, -1)}=${mysql.escape(yomi)}`
          )
          .catch(console.error));
      if (!isAdded) return message.reply("辞書を追加(編集)できませんでした。");
      message.reply("正常に辞書を追加(編集)できました。");
      break;
    case "delete":
      [word] = args;
      const isDeleted = await dicts
        .queryPromise(
          `alter table \`${message.guild.id}\` drop column ${mysql
            .escape(word)
            .slice(1, -1)}`
        )
        .catch(console.error);
      if (isDeleted) message.reply("正常に辞書を削除できました。");
      else message.reply("辞書を削除出来ませんでした。");
      break;
    default:
      message.reply("サブコマンドが違います。");
  }
};
