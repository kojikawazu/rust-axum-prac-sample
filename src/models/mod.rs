// モデルのモジュールの宣言
pub mod users;
pub mod auth;
pub mod common;

// 共通の型をre-export
pub use common::common::{NaiveDateTimeWrapper, UuidWrapper};

// モデルのエントリーポイント
