# Release Process

`go2rust-cli` は Major.Minor.Patch の SemVer でバージョンを管理します。

## Version Rule

- `Cargo.toml` の `package.version` は `MAJOR.MINOR.PATCH` にする。
- Git tag は先頭に `v` を付けて `vMAJOR.MINOR.PATCH` にする。
- 例: `Cargo.toml` が `0.1.0` の場合、tag は `v0.1.0`。
- tag と `Cargo.toml` の version が一致しない場合、Release workflow は失敗する。

## Release

Release は tag push で作成します。

```sh
jj git fetch --remote origin
jj bookmark set main -r main@origin
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions が次を生成します。

- GitHub Release
- Linux x86_64 の release archive
- archive の SHA-256 checksum
- GitHub Container Registry package

## Package

Container image は GitHub Container Registry に publish されます。

```sh
docker pull ghcr.io/kazunari-kamata/go2rust-cli:0.1.0
```

実行例:

```sh
docker run --rm -v "$PWD:/workspace" ghcr.io/kazunari-kamata/go2rust-cli:0.1.0 convert -i examples/hello.go
```

