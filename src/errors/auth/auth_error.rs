// 必要なクレートのインポート
use axum::http::StatusCode;

// 認証エラーの列挙型
#[derive(Debug)]
pub enum AuthError {
    // 無効な資格情報
    InvalidCredentials,
    // 無効なトークン
    InvalidToken,
    // トークン作成エラー
    TokenCreationError(String),
    // データベースエラー
    DatabaseError(String),
    // ユーザーが見つかりません
    UserNotFound,
}

// エラーをHTTPステータスコードとメッセージに変換
impl From<AuthError> for (StatusCode, String) {
    fn from(error: AuthError) -> Self {
        match error {
            // 無効な資格情報
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()),
            // 無効なトークン
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            // トークン作成エラー
            AuthError::TokenCreationError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Token creation error: {}", e)),
            // データベースエラー
            AuthError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)),
            // ユーザーが見つかりません
            AuthError::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
        }
    }
}