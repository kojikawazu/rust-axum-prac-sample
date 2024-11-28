# Rust + Webフレームワークによるバックエンド開発(学習用)

## Summary

- ユーザーのCRUD操作を行うAPIを作成する

## Tech Stack

[![Rust](https://img.shields.io/badge/Rust-1.74.0-orange.svg)](https://www.rust-lang.org/)
[![Supabase](https://img.shields.io/badge/Supabase-2.46.0-blue.svg)](https://supabase.com/)
[![Axum](https://img.shields.io/badge/Axum-0.7.11-green.svg)](https://github.com/tokio-rs/axum)

# Webフレームワーク

Axumを使用してAPIを作成する。

Axumとは・・・

- Webフレームワークの1つ
- 非同期処理をサポートしている
- ミドルウェアをサポートしている
- テストコードをサポートしている

## データベース

- Supabaseを使用してデータベースを管理する
- JWTを使用して認証を行う
- テストコードを作成する

## プロジェクトの操作

### 1. プロジェクトのビルド

```bash
cargo build
```

### 2. プロジェクトの実行

```bash
cargo run
```

### 3. プロジェクトのテスト

```bash
cargo test
```

### Other

TODO
