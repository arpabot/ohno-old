# ohno

## これはなに
Azure Cognitive ServiceのText to Speechを利用した読み上げBot<br>

セルフホストめんどくさい系の人のためのアレ: [導入](https://discord.com/api/oauth2/authorize?client_id=919024472936288287&permissions=274981719296&scope=bot)

## 動かし方
.envをいじれ

```properties
token=your Discord Bot token
prefix=prefix
apiKey=Azure Cognitive Service Api Key
region=Azure Cognitive Service Region
```
そして起動

```bash
npm start
```

## コマンドのガイド
- コマンド名
  - 説明
  - エイリアス(別名)
<br>
- connect
  - 接続して読み上げを開始します
  - s, start, con, c, join
- disconnect
  - 切断して読み上げを終了します
  - e, end, dis, dc, leave
