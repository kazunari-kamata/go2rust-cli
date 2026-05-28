# go2rust-cli Specification

このドキュメントは `go2rust-cli` の現在仕様と今後の拡張方針をまとめるものです。

## 目的

Go ファイルを入力として受け取り、Rust ファイルまたは標準出力へ Rust ソースコードの雛形を出力する CLI を提供する。

初期版は完全な Go to Rust 変換器ではなく、機械的に置き換えやすい構文だけを変換する。未対応構文は Rust コード内に TODO コメントとして残し、後続の手作業や将来の自動変換で追跡できるようにする。

## 非目標

- Go の完全な構文解析
- Go と Rust の完全な型システム対応
- Go の標準ライブラリから Rust crate への自動対応付け
- 実行可能な Rust コードの保証
- goroutine、channel、interface、method の完全変換

## CLI

### convert

```sh
go2rust-cli convert -i input.go -o output.rs
go2rust-cli convert --input input.go --output output.rs
go2rust-cli convert -i input.go
go2rust-cli convert -i input.go --check
```

### オプション

- `-i`, `--input`: 入力 Go ファイル。必須。
- `-o`, `--output`: 出力 Rust ファイル。任意。
- `--check`: 変換可能かだけを確認する。出力ファイルは書かない。

### 出力先

- `--output` 指定あり: 指定ファイルへ書き込む。
- `--output` 指定なし: 標準出力へ書き込む。
- `--check` 指定あり: 成功時は `OK` を出力する。

## 変換仕様

### package

入力:

```go
package main
```

出力:

```rust
// Go package: main
```

### import

入力:

```go
import "fmt"
```

出力:

```rust
// Go import: fmt
```

### main function

入力:

```go
func main() {
}
```

出力:

```rust
fn main() {
}
```

### named function with parameters and return type

入力:

```go
func add(a, b int) int {
}
```

出力:

```rust
fn add(a: i32, b: i32) -> i32 {
}
```

### fmt.Println

入力:

```go
fmt.Println("hello")
```

出力:

```rust
println!("hello");
```

### fmt.Print

入力:

```go
fmt.Print("hello")
```

出力:

```rust
print!("hello");
```

### string variable declaration

入力:

```go
var name string
```

出力:

```rust
let mut name: String;
```

### variable declaration with initializer

入力:

```go
var name string = "value"
var enabled bool = true
var total float64 = 1.5
```

出力:

```rust
let mut name: String = "value";
let mut enabled: bool = true;
let mut total: f64 = 1.5;
```

### int variable declaration with initializer

入力:

```go
var count int = 1
```

出力:

```rust
let mut count: i32 = 1;
```

### short variable declaration

入力:

```go
count := 1
```

出力:

```rust
let mut count = 1;
```

### assignment

入力:

```go
count = count + 1
```

出力:

```rust
count = count + 1;
```

### simple function call statement

入力:

```go
logIfEnabled(true)
```

出力:

```rust
logIfEnabled(true);
```

### simple if block

入力:

```go
if count > 0 {
}
```

出力:

```rust
if count > 0 {
}
```

### simple else if / else block

入力:

```go
if count > 10 {
} else if count > 0 {
} else {
}
```

出力:

```rust
if count > 10 {
} else if count > 0 {
} else {
}
```

### simple conditional for loop

入力:

```go
for count > 0 {
}
```

出力:

```rust
while count > 0 {
}
```

### infinite for loop

入力:

```go
for {
}
```

出力:

```rust
loop {
}
```

### return

入力:

```go
return x
```

出力:

```rust
return x;
```

入力:

```go
return
```

出力:

```rust
return;
```

### unsupported line

入力:

```go
switch count {
```

出力:

```rust
// TODO(go2rust): original line: switch count {
```

## 内部設計

現在は行単位の簡易変換を行う。

1. `parser` が Go ソースを行ごとに読み、正規表現で `IrItem` へ変換する。
2. `generator` が `IrItem` から Rust ソース文字列を生成する。
3. `main` がファイル入出力、標準出力、`--check` の制御を行う。

## テスト方針

- 対応済み構文は期待する Rust 出力を完全一致でテストする。
- 未対応構文は `TODO(go2rust)` コメントとして残ることをテストする。
- 変換ルールを追加する場合は、正常系と未対応構文への影響を確認する。
- CLI の入出力仕様を変更する場合は、可能であれば integration test を追加する。

## 拡張候補

優先度の高い候補:

- import ブロック
- 3句 `for`
- `range`
- 複数戻り値
- 配列、スライス、map の基本変換

将来的な候補:

- Go AST を使った解析
- Rust AST または formatter との連携
- 変換できなかった構文のレポート出力
- 型対応表の外部設定化
- 複数ファイル入力
- ディレクトリ単位変換

## 互換性方針

- CLI の既存オプションは破壊的に変更しない。
- TODO コメント形式は後続ツールが利用できるように維持する。
- 出力の基本インデントは4スペースを維持する。
- 既存テストの期待出力を変える場合は、仕様変更として README とこのドキュメントも更新する。
