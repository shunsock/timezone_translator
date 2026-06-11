# presentation

tzt のプレゼンテーション層。
CLI との境界を担います。

## 概要

公開するのは `run() -> ExitCode` 1つです。
ルートの `tzt` バイナリは、これを呼ぶだけです。

内部は2つのモジュールに分かれます。

| モジュール | 責務 |
|-----------|------|
| `command` | clap によるコマンド定義と引数の受け取り |
| `validator` | 文字列を domain の VO へ写す |

## 背景

CLI の関心事 (フラグ名、ヘルプ文、終了コード、エラー表示) は変わりやすいものです。
変換ロジックと混ざると、どちらの変更も難しくなります。

## 目的

「外界との会話」をこの層に閉じ込めること。
domain / usecase に文字列や clap を持ち込ませないこと。

## 手法

処理は一直線です。

1. `receive_user_input()` — clap で引数を受け取る
2. `validate_command_options()` — 各文字列を `.parse()` で VO に写す
3. `TimezoneTranslator::new(request).convert()` — usecase に委譲
4. 結果を stdout / stderr に出し分け、`ExitCode` を返す

検証ロジックはこの層にありません。
`.parse()` の向こう側、domain の `FromStr` がすべてを担います。

`--from` / `--to` のデフォルト値には、起動時に infrastructure から
取得したローカルタイムゾーンを渡します。

## 処理の事例

```
$ tzt -T "2024-01-01 12:00:00" -f UTC -t Asia/Tokyo
2024-01-01 21:00:00 JST

$ tzt -T "bad input"
Validation Error: Invalid time format found. bad input (expected: YYYY-MM-DD hh:mm:ss)
# 終了コード 1
```

## 依存

domain / usecase / infrastructure のすべてに依存する、唯一の層です。
依存方向は常にこの層から下へ向かいます。
