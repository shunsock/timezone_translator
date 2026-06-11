# domain

tzt のドメイン層。
タイムゾーン変換という問題を、型で表現するクレートです。

## 概要

バリューオブジェクト (VO) だけを置いています。
ロジックの正しさを、型の存在で保証します。

| 型 | 意味 |
|----|------|
| `ConversionTime` | 変換対象の壁時計時刻 |
| `SourceTimezone` | 入力時刻が属するタイムゾーン |
| `TargetTimezone` | 変換先のタイムゾーン |
| `AmbiguousTimeStrategy` | 曖昧な時刻の解決方針 (earliest / latest) |
| `TranslationRequest` | 上記4つの集約。検証済みの変換リクエスト |

## 背景

かつて入力検証は、presentation 層の関数群に散らばっていました。
「検証済みかどうか」は型に現れませんでした。
`Tz` を2つ取る関数では、from と to を入れ替えてもコンパイルが通りました。

## 目的

不正な状態を、型で表現不可能にすること。
Make Illegal States Unrepresentable.

## 手法

**Parse, don't validate.**

各 VO は `FromStr` でしか構築できません。
構築できた = 検証を通った、が型レベルで成立します。

`SourceTimezone` と `TargetTimezone` は別の型です。
取り違えはコンパイルエラーになります。

各 VO は自分のパースエラー型を持ちます。
ユーザー向けメッセージはエラー型に同居しています。

## 処理の事例

```rust
use domain::{
    AmbiguousTimeStrategy, ConversionTime,
    SourceTimezone, TargetTimezone, TranslationRequest,
};

// 文字列から VO を構築する。失敗はエラー型で返る
let time: ConversionTime = "2024-01-01 12:00:00".parse()?;
let source: SourceTimezone = "America/New_York".parse()?;
let target: TargetTimezone = "UTC".parse()?;
let strategy: AmbiguousTimeStrategy = "earliest".parse()?;

// すべて揃ったときだけリクエストが作れる
let request = TranslationRequest::new(time, source, target, strategy);
```

受理する時刻形式は3つです。

- `2024-01-01 12:00:00`
- `2024-01-01T12:00:00` (ISO 8601)
- `2024-01-01` (00:00:00 を補完)

形式の定義は `conversion_time.rs` の `ACCEPTED_FORMATS` テーブルにあります。

## 依存

他レイヤーに依存しません。
chrono / chrono-tz / regex / thiserror のみを使います。
