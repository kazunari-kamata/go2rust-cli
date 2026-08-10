# 2026-08-10

## 実施内容
- 現在の `main` 状態を確認し、simple switch / condition switch の変換が merge 済みであることを確認。
- README と `docs/spec.md` の対応済み構文・未対応構文を確認し、残課題を現在仕様に合わせて更新。
- import ブロック対応を実装。`import (...)` 内の通常 import spec (`"fmt"` など) を import ごとの `// Go import: ...` コメントとして出力するよう `src/parser.rs` を更新。
- alias import、blank import、dot import は自動変換せず、`TODO(go2rust)` として情報を保持するテストを追加。
- README、`docs/spec.md`、`AGENTS.md` を import ブロック対応済みの仕様に更新。
- `go2rust-cli-samples` に import ブロック確認用サンプルを追加。
- `cargo fmt -- --check`、`cargo clippy -- -D warnings`、`cargo test`、`cargo run -- convert -i examples/hello.go --check` が成功。
- samples 側で `go test ./...` と `cargo run --manifest-path ../go2rust-cli/Cargo.toml -- convert -i samples/basic/import_blocks/main.go --check` が成功。

## 未完了タスク
- import の追加課題:
  - alias import、blank import、dot import は現在 `TODO(go2rust)` として残る。Rust 側の出力方針を決める必要がある。
- 3句 `for` 対応:
  - `for i := 0; i < n; i++ { ... }` を Rust 風の雛形へ変換する。
  - 初期案は `let mut i = 0; while i < n { ...; i += 1; }` 相当。ただし post statement の配置でブロック末尾処理が必要。
- `range` 対応:
  - `for _, value := range values { ... }` の最小変換を追加する。
  - slice/array と map で Rust 側の雛形が変わるため、まずは TODO を保持しつつ情報を落とさない段階実装も検討する。
- switch の追加課題:
  - `fallthrough` は現在 `TODO(go2rust)` として残る。Rust の `match` では意味が異なるため、自動変換する場合は明示的な方針が必要。
  - type switch、複雑な case 条件、式変換が必要な case は未対応。
- 型・構造の拡張:
  - 複数戻り値、配列、スライス、map、struct、method、interface は未対応。
  - goroutine、channel、Go の error idiom は未対応。

## 次回作業
- 次の優先候補は 3句 `for` または `range`。どちらもブロック末尾に追加処理が必要になり得るため、先に期待出力を `tests/convert_test.rs` に固定する。
- 3句 `for` 対応時は `go2rust-cli-samples` に `samples/basic/for_clauses/main.go` のような最小サンプルを追加する。
- `range` 対応時は `go2rust-cli-samples` の既存 `samples/unsupported/control_flow/main.go` を対応済みサンプルへ移すか、新規 basic sample を追加する。
- 検証は `cargo fmt -- --check`、`cargo clippy -- -D warnings`、`cargo test` を実行する。
- samples 側の検証は `go test ./...` と、追加サンプルに対する `cargo run --manifest-path ../go2rust-cli/Cargo.toml -- convert -i <sample> --check` を実行する。

---

# 2026-07-02

## 実施内容
- PR conflict 解消のため `main@origin` へ rebase。既に main に入った simple switch 変更との重複を解消し、condition switch 追加分を残した。
- rebase 後に `cargo fmt -- --check`、`cargo test`、`cargo clippy -- -D warnings` を再実行し成功。
- jj bookmark `codex/add-simple-switch-conversions` を origin へ push し、draft PR https://github.com/kazunari-kamata/go2rust-cli/pull/10 を作成。
- Go toolchain インストール後、`go2rust-cli-samples` の `go test ./...` を実行し成功。
- 併せて `cargo test` を再実行し、10 tests すべて成功。
- 既存途中変更の単純な `switch expr` 変換を確認し、残TODOとして expression なし `switch` の変換を追加。
- `switch { case cond: ... default: ... }` を `match () { _ if cond => { ... } _ => { ... } }` へ変換するよう `parser` / `generator` の block tracking を調整。
- `case a, b:` は expression なし `switch` では `_ if a || b`、expression あり `switch` では `a | b` として出力。
- `fallthrough` は未対応として `TODO(go2rust)` に残るテストを追加。
- README と `docs/spec.md` に condition switch の仕様を追記。
- `cargo fmt -- --check`、`cargo clippy -- -D warnings`、`cargo test` を実行し成功。
- `go2rust-cli-samples` の追加サンプルを `cargo run --manifest-path ../go2rust-cli/Cargo.toml -- convert` で変換確認。

## 未完了タスク
- 2026-08-10 の残課題一覧に更新済み。

## 次回作業
- 2026-08-10 の次回作業に更新済み。
