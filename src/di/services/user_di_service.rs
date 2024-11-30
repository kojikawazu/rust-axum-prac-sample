use std::sync::Arc;
use uuid::Uuid;
// リポジトリ
use crate::di::repositories::user_repository::{UserRepositoryTrait};
// ユーザー
use crate::models::users::users::{User, NewUser};
// エラー
use crate::errors::users::user_error::UserError;

// サービス
pub struct UserDIService {
    repository: Arc<dyn UserRepositoryTrait>,
}

// メソッド
impl UserDIService {
    // コンストラクタ
    pub fn new(repository: Arc<dyn UserRepositoryTrait>) -> Self {
        Self { repository }
    }

    // 全件取得
    pub async fn get_all_users(&self) -> Result<Vec<User>, UserError> {
        self.repository.find_all().await
    }

    // 1件取得
    pub async fn get_user_by_id(&self, id: Uuid) -> Result<User, UserError> {
        self.repository.find_by_id(id).await
    }

    // 作成
    pub async fn create_user(&self, user: NewUser) -> Result<User, UserError> {
        self.repository.create(user).await
    }

    // 更新
    pub async fn update_user(&self, id: Uuid, user: NewUser) -> Result<User, UserError> {
        self.repository.update(id, user).await
    }

    // 削除
    pub async fn delete_user(&self, id: Uuid) -> Result<(), UserError> {
        self.repository.delete(id).await
    }
}