# ohno

## これはなに
Azure Cognitive ServiceのText to Speechを利用した読み上げBot<br>

Pre-Hosted: [導入](https://discord.com/api/oauth2/authorize?client_id=946032263915782184&permissions=3148800&scope=bot%20applications.commands)

## セルフホスト

### 必要なもの
- autoconf 
- automake
- libtool
- libssl-dev 
- libtool

### .envを編集する
```properties
token=your Discord Bot token
prefix=prefix
apiKey=Azure Cognitive Service Api Key
region=Azure Cognitive Service Region
```

### 起動
```bash
cargo run --release
```

## コマンドのガイド
- スラッシュコマンド対応です

| コマンド | 説明 | 備考 |
| - | - | - |
| /help | コマンドのヘルプを表示します | registerは見れません |
| /connect | ボイスチャンネルに接続します |  |
| /leave | ボイスチャンネルから退出します |  |