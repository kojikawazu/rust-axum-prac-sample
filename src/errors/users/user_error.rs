// 必要なクレートのインポート
use axum::http::StatusCode;
use thiserror::Error;

// ユーザーエラーの列挙型
#[derive(Error, Debug)]
pub enum UserError {
    // データベースエラー
    #[error("データベースエラー: {0}")]
    DatabaseError(String),
    #[error("ユーザーが見つかりません")]
    UserNotFound,
    // 不正なデータ形式
    #[error("不正なデータ形式: {0}")]
    InvalidData(String),
    // パスワード処理エラー
    #[error("パスワード処理エラー: {0}")]
    PasswordError(String),
    // JSONエラー
    #[error("JSONエラー: {0}")]
    JsonError(String),
}

// エラーをHTTPステータスコードとメッセージに変換
impl From<UserError> for (StatusCode, String) {
    fn from(err: UserError) -> Self {
        match err {
            // データベースエラー
            UserError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            // ユーザーが見つかりません
            UserError::UserNotFound => (StatusCode::NOT_FOUND, "ユーザーが見つかりません".to_string()),
            // 不正なデータ形式
            UserError::InvalidData(msg) => (StatusCode::BAD_REQUEST, msg),
            // パスワード処理エラー
            UserError::PasswordError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            // JSONエラー
            UserError::JsonError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        }
    }
}