# usecase

tzt のユースケース層。
「時刻をタイムゾーン間で変換する」操作そのものを担います。

## 概要

公開するのは `TimezoneTranslator` 1つです。
入力は `domain::TranslationRequest`、出力は `Result<DateTime<Tz>, TranslationError>` です。

## 背景

タイムゾーン変換は単純な足し算ではありません。
DST (夏時間) の切り替わりで、2つの特殊ケースが生まれます。

1. **曖昧な時刻** — DST 終了日は同じ壁時計時刻が2回現れる
2. **存在しない時刻** — DST 開始日は時計が飛び、間の時刻が消える

この分岐の扱いが、このモジュールの存在理由です。

## 目的

変換ロジックを1箇所に集めること。
CLI やシステム環境の事情から切り離すこと。

## 手法

入力は検証済みの `TranslationRequest` だけを受け取ります。
このモジュールに文字列検証はありません。

chrono の `LocalResult` の3分岐をそのまま写し取ります。

- `Single` — 一意に決まる。そのまま変換
- `Ambiguous` — リクエストの戦略 (earliest / latest) で選ぶ
- `None` — `TranslationError::NonexistentTime` を返す

失敗は `Result` で呼び出し側に強制します (鉄道指向)。

## 処理の事例

```rust
use usecase::TimezoneTranslator;

// New York 2024-11-03 01:30 は DST 終了で2回現れる曖昧な時刻
let request = /* domain::TranslationRequest (strategy: latest) */;

let translated = TimezoneTranslator::new(request).convert()?;
// => 2024-11-03 06:30:00 UTC (2回目の 01:30 を採用)
```

## 依存

domain にのみ依存します (+ chrono / chrono-tz / thiserror)。
infrastructure や presentation を知りません。
