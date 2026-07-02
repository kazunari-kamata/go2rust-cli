# go2rust-cli

Go ソースコードを Rust ソースコードの雛形へ変換する CLI です。

初期版では完全な言語変換ではなく、対応済みの単純な構文だけを Rust 風のコードへ変換します。未対応の Go 構文は Rust コード内に `TODO(go2rust)` コメントとして残します。

## インストール方法

Rust toolchain をインストールした環境で、このプロジェクトのルートからビルドします。

```sh
cargo build --release
```

生成されたバイナリは `target/release/go2rust-cli` です。

開発中は次のように直接実行できます。

```sh
cargo run -- convert -i examples/hello.go
```

## 使用例

入力ファイルを指定して標準出力へ変換結果を出します。

```sh
go2rust-cli convert -i input.go
```

出力ファイルを指定します。

```sh
go2rust-cli convert -i input.go -o output.rs
go2rust-cli convert --input input.go --output output.rs
```

変換可能かだけを確認します。

```sh
go2rust-cli convert -i input.go --check
```

## Releases / Packages

バージョンは Major.Minor.Patch の SemVer で管理します。

- `Cargo.toml`: `0.1.0`
- Git tag: `v0.1.0`

`vMAJOR.MINOR.PATCH` 形式の tag を push すると、GitHub Actions が GitHub Release と GitHub Container Registry package を生成します。

```sh
git tag v0.1.0
git push origin v0.1.0
```

Container image の利用例です。

```sh
docker run --rm -v "$PWD:/workspace" ghcr.io/kazunari-kamata/go2rust-cli:0.1.0 convert -i examples/hello.go
```

詳細は [docs/release.md](docs/release.md) を参照してください。

## 対応済み構文

- `package main` をコメントとして出力
- `import "fmt"` をコメントとして出力
- `func main() { ... }` を `fn main() { ... }` に変換
- `func add(a, b int) int { ... }` を `fn add(a: i32, b: i32) -> i32 { ... }` に変換
- `fmt.Print("hello")` を `print!("hello");` に変換
- `fmt.Println("hello")` を `println!("hello");` に変換
- `var name string` を `let mut name: String;` に変換
- `var name string = "value"` を `let mut name: String = "value";` に変換
- `var count int = 1` を `let mut count: i32 = 1;` に変換
- `var enabled bool = true` を `let mut enabled: bool = true;` に変換
- `var total float64 = 1.5` を `let mut total: f64 = 1.5;` に変換
- `count := 1` を `let mut count = 1;` に変換
- `count = count + 1` を `count = count + 1;` に変換
- `logIfEnabled(true)` を `logIfEnabled(true);` に変換
- `if count > 0 { ... }` を `if count > 0 { ... }` に変換
- `} else if count > 0 {` を `} else if count > 0 {` に変換
- `} else {` を `} else {` に変換
- `for { ... }` を `loop { ... }` に変換
- `for count > 0 { ... }` を `while count > 0 { ... }` に変換
- `switch count { case 1: ... default: ... }` を `match count { 1 => { ... } _ => { ... } }` に変換
- `switch { case count > 0: ... default: ... }` を `match () { _ if count > 0 => { ... } _ => { ... } }` に変換
- `return x` を `return x;` に変換
- `return` を `return;` に変換

## 未対応構文

次のような構文は初期版では変換せず、`TODO(go2rust)` コメントとして出力します。

- 3句 `for`、`range`
- fallthrough、複雑な case 条件
- 複雑な条件式
- 構造体、インターフェース、メソッド
- 複数 import や import ブロック
- 複雑な Go の型推論
- goroutine、channel
- エラー処理の自動変換
- パッケージ分割やモジュール変換

## 今後の拡張方針

- Go の AST を使った堅牢なパースへの移行
- 変換対象構文の段階的な追加
- 型変換ルールの拡充
- import と依存関係の Rust crate への対応付け
- 変換結果の整形と `rustfmt` 連携
- 未対応構文レポートの出力
