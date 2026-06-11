# infrastructure

tzt のインフラ層。
実行中のシステムから、ローカルタイムゾーン名を検出します。

## 概要

公開するのは `provide_local_timezone_string()` 1つです。
`"Asia/Tokyo"` のような IANA 名を `String` で返します。

## 背景

tzt は `--from` / `--to` を省略できます。
省略時のデフォルトは「いまいる場所のタイムゾーン」です。

しかし Unix システムにその答えの置き場所は複数あります。
環境・ディストリビューションによって、どれが使えるかが違います。

## 目的

「システムに聞く」という副作用をこの層に閉じ込めること。
domain / usecase を環境の事情から自由に保つこと。

## 手法

3つの情報源を、信頼できる順に試します。

1. 環境変数 `TZ`
2. `/etc/localtime` のシンボリックリンク先 (zoneinfo パスから抽出)
3. `/etc/timezone` の中身 (Debian 系)

実装は `or_else` チェーンです。
優先順位がコードの字面と一致します。

すべて失敗した場合は panic します。
デフォルト値を提供できない以上、起動を続ける意味がないためです。

## 処理の事例

```rust
use infrastructure::provide_local_timezone_string;

// TZ=Asia/Tokyo が設定された環境なら
let local = provide_local_timezone_string();
// => "Asia/Tokyo"
```

## 依存

他レイヤーに依存しません。
std のみで動きます (テストのみ regex を使用)。
