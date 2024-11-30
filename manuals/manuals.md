# Rustの環境構築

## Rust のインストール

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 環境変数の設定

```bash
source $HOME/.cargo/env
```

## インストールの確認

```bash
rustc --version
cargo --version
```

## プロジェクトの作成

```bash
cargo new backend
cd backend
```

## プロジェクトをビルドして実行

```bash
# ビルド
cargo build
# 実行
cargo run
```

## 依存ライブラリの追加

Cargo.tomlに追加したいライブラリを記述する。
その後ビルドして実行する。

```toml
[dependencies]
# ここに追加したいライブラリを記述する
```

## テスト

```bash
# 全てのテスト
cargo test
# ユーザーサービスのテスト
cargo test user_services_test
# ユーザーサービスの統合テスト
cargo test --test user_services_integration_test
# テストのログを出力
RUST_LOG=debug cargo test test_create -- --nocapture
```
