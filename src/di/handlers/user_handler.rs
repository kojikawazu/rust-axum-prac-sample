use axum::{
    extract::Json,
    http::StatusCode,
};
use std::sync::Arc;
use uuid::Uuid;
// ユーザー
use crate::models::users::users::{User, NewUser};
// エラー
use crate::errors::users::user_error::UserError;
// サービス
use crate::di::services::user_di_service::UserDIService;

// ハンドラー
pub struct UserHandler {
    service: Arc<UserDIService>,
}

// メソッド
impl UserHandler {
    // コンストラクタ
    pub fn new(service: Arc<UserDIService>) -> Self {
        Self { service }
    }

    pub async fn get_users(
        &self,
    ) -> Result<Json<Vec<User>>, (StatusCode, String)> {
        self.service
            .get_all_users()
            .await
            .map(Json)
            .map_err(|e| {
                match e {
                    UserError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                    UserError::UserNotFound => (StatusCode::NOT_FOUND, "ユーザーが見つかりません".to_string()),
                    UserError::InvalidData(msg) => (StatusCode::BAD_REQUEST, msg),
                    UserError::PasswordError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                    UserError::JsonError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                }
            })
    }

    pub async fn get_user(
        &self,
        id: Uuid,
    ) -> Result<Json<User>, (StatusCode, String)> {
        self.service
            .get_user_by_id(id)
            .await
            .map(Json)
            .map_err(|e| {
                match e {
                    UserError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                    UserError::UserNotFound => (StatusCode::NOT_FOUND, "ユーザーが見つかりません".to_string()),
                    UserError::InvalidData(msg) => (StatusCode::BAD_REQUEST, msg),
                    UserError::PasswordError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                    UserError::JsonError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                }
            })
    }

    pub async fn create_user(
        &self,
        Json(user): Json<NewUser>,
    ) -> Result<Json<User>, (StatusCode, String)> {
        self.service
            .create_user(user)
            .await
            .map(Json)
            .map_err(|e| {
                match e {
                    UserError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                    UserError::UserNotFound => (StatusCode::NOT_FOUND, "ユーザーが見つかりません".to_string()),
                    UserError::InvalidData(msg) => (StatusCode::BAD_REQUEST, msg),
                    UserError::PasswordError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                    UserError::JsonError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                }
            })
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        Json(user): Json<NewUser>,
    ) -> Result<Json<User>, (StatusCode, String)> {
        self.service
            .update_user(id, user)
            .await
            .map(Json)
            .map_err(|e| {
                match e {
                    UserError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                    UserError::UserNotFound => (StatusCode::NOT_FOUND, "ユーザーが見つかりません".to_string()),
                    UserError::InvalidData(msg) => (StatusCode::BAD_REQUEST, msg),
                    UserError::PasswordError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                    UserError::JsonError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                }
            })
    }

    pub async fn delete_user(
        &self,
        id: Uuid,
    ) -> Result<StatusCode, (StatusCode, String)> {
        self.service
            .delete_user(id)
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(|e| {
                match e {
                    UserError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                    UserError::UserNotFound => (StatusCode::NOT_FOUND, "ユーザーが見つかりません".to_string()),
                    UserError::InvalidData(msg) => (StatusCode::BAD_REQUEST, msg),
                    UserError::PasswordError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                    UserError::JsonError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                }
            })
    }
}