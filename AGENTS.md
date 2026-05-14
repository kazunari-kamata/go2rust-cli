# AGENTS.md

このディレクトリは `go2rust-cli` プロジェクトです。Codex などのコーディングエージェントは、このファイルと `docs/spec.md` を優先して参照してください。

## プロジェクト概要

`go2rust-cli` は Go ソースコードを Rust ソースコードの雛形へ変換する Rust 製 CLI です。

初期版の目的は完全変換ではありません。対応済みの単純な Go 構文だけを Rust 風のコードへ変換し、未対応の Go 構文は Rust 出力内に `TODO(go2rust)` コメントとして残します。

## 技術方針

- Rust 2021 edition を維持する。
- CLI 引数は `clap` で実装する。
- アプリケーション境界のエラー処理は `anyhow` を使う。
- ドメイン固有エラーは `thiserror` を使う。
- 初期版の簡易変換は `regex` を使う。
- 変換ロジックは `parser`、`ir`、`generator` に分ける。
- テストから変換関数を直接呼べるように、公開APIは `src/lib.rs` に置く。
- 未対応構文を黙って捨てない。必ず `// TODO(go2rust): original line: ...` として出力する。

## ファイル構成

- `src/main.rs`: CLI エントリポイント、入出力、標準出力、`--check`
- `src/cli.rs`: `clap` の引数定義
- `src/parser.rs`: Go の行単位簡易解析
- `src/ir.rs`: 中間表現
- `src/generator.rs`: Rust 雛形生成
- `src/error.rs`: ドメインエラー
- `src/lib.rs`: 変換APIとモジュール公開
- `examples/hello.go`: 最小サンプル
- `tests/convert_test.rs`: 変換の統合テスト
- `.github/workflows/ci.yml`: CI
- `docs/spec.md`: 現在の仕様と拡張方針

## CLI仕様

維持するインターフェース:

```sh
go2rust-cli convert -i input.go -o output.rs
go2rust-cli convert --input input.go --output output.rs
go2rust-cli convert -i input.go
go2rust-cli convert -i input.go --check
```

挙動:

- `-o` / `--output` が未指定の場合は標準出力へ Rust コードを出す。
- `--check` は入力の読み込みと変換処理だけを確認し、出力ファイルを書かない。
- `--check` 成功時は `OK` を出力する。

## 現在対応している変換

- `package main` -> `// Go package: main`
- `import "fmt"` -> `// Go import: fmt`
- `func main() { ... }` -> `fn main() { ... }`
- `fmt.Println("hello")` -> `println!("hello");`
- `var name string` -> `let mut name: String;`
- `var count int = 1` -> `let mut count: i32 = 1;`
- `return x` -> `return x;`

## 未対応構文の扱い

未対応行は必ず次の形式で出力する。

```rust
// TODO(go2rust): original line: ...
```

未対応ブロックを追加対応する場合は、閉じ波括弧の扱いで出力のブロック構造が壊れないようにテストを追加する。

## 作業時の検証

変更後は可能な限り次を実行する。

```sh
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo run -- convert -i examples/hello.go
```

CI と同じ品質ゲートは `cargo fmt -- --check`、`cargo clippy -- -D warnings`、`cargo test`。

## バージョン管理

- このプロジェクトは GitHub 上では Git リポジトリとして公開する。
- ローカル作業はできるだけ `jj` を使う。
- 既存 Git リポジトリでは `jj git init --colocate .` 済みの colocated repo として扱う。
- リモート同期は `jj git fetch` と `jj git push` を優先する。
- 履歴確認は `jj log`、状態確認は `jj status`、差分確認は `jj diff` を優先する。
- ブックマーク操作は `jj bookmark` を使う。`main` は `main@origin` をtrackする。
- `git` は `jj` で扱いづらい GitHub CLI 連携や緊急確認の補助に限定する。

## 実装時の注意

- 新しい変換を足す場合は、まず `tests/convert_test.rs` に期待出力を追加する。
- 変換ルールが増えても、初期版では複雑なパーサを導入しない。`docs/spec.md` の拡張方針に沿って段階的に進める。
- Regex は行全体にマッチする形を基本にし、想定外の部分一致で誤変換しない。
- Rust 出力の末尾には改行を付ける。
- README は利用者向け、`AGENTS.md` と `docs/spec.md` は開発者・エージェント向けとして役割を分ける。
- ローカル環境固有の絶対パスやユーザー名をドキュメント、コメント、コミットメッセージに含めない。
