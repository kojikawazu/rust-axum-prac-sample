// 必要なクレートのインポート
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

// ユーザーモデルの定義
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct User {
    // ユーザーID
    #[schema(value_type = UuidWrapper)]
    pub id: Uuid,
    // ユーザー名
    #[schema(example = "John Doe")]
    pub username: String,
    // メールアドレス
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    // パスワード
    #[schema(example = "password123")]
    pub password: String,
    // 作成日時
    #[schema(value_type = NaiveDateTimeWrapper)]
    pub created_at: NaiveDateTime,
    // 更新日時
    #[schema(value_type = NaiveDateTimeWrapper)]
    pub updated_at: NaiveDateTime,
}

// 新しいユーザーモデルの定義
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct NewUser {
    // ユーザー名
    #[schema(example = "John Doe")]
    pub username: String,
    // メールアドレス
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    // パスワード
    #[schema(example = "password123")]
    pub password: String,
}
