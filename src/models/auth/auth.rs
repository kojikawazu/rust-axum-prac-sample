// 必要なクレートのインポート
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Duration, Utc};
use utoipa::ToSchema;
// ユーザーモデルのインポート
use crate::models::users::users::User;

// サインイン資格情報
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SignInCredentials {
    // メールアドレス
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    // パスワード
    #[schema(example = "password123")]
    pub password: String,
}

// JWTクレーム
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    // ユーザーID
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub sub: String,
    // 有効期限
    #[schema(example = "1727364545")]
    pub exp: i64,
    // 発行時間
    #[schema(example = "1727364545")]
    pub iat: i64,
}

// 認証レスポンス
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    // JWTトークン
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwiaWF0IjoxNzI3MzY0NTQ1LCJleHAiOjE3MjczNjQ1NDV9.6e5e6e5e6e5e6e5e6e5e6e5e6e5e6e5e6e5e6e5e")]
    pub token: String,
    // ユーザー
    #[schema(example = "John Doe")]
    pub user: User,
}

// JWTクレームの生成
impl Claims {
    // JWTクレームの新しいインスタンスを作成
    pub fn new(user_id: &Uuid) -> Self {
        // 現在の時刻を取得
        let now = Utc::now();
        // JWTクレームの新しいインスタンスを作成
        Self {
            sub: user_id.to_string(),
            exp: (now + Duration::hours(24)).timestamp(),
            iat: now.timestamp(),
        }
    }
}