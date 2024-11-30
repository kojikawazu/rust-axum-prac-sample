use async_trait::async_trait;
use uuid::Uuid;
use chrono::Utc;
use reqwest::Client;
use bcrypt::{hash, DEFAULT_COST};

// ユーザー
use crate::models::users::users::{User, NewUser};
// エラー
use crate::errors::users::user_error::UserError;

// トレイト
#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn find_all(&self) -> Result<Vec<User>, UserError>;
    async fn find_by_id(&self, id: Uuid) -> Result<User, UserError>;
    async fn create(&self, user: NewUser) -> Result<User, UserError>;
    async fn update(&self, id: Uuid, user: NewUser) -> Result<User, UserError>;
    async fn delete(&self, id: Uuid) -> Result<(), UserError>;
}

// リポジトリ
pub struct UserRepository {
    // クライアント 
    client: Client,
    // SupabaseのURL
    supabase_url: String,
    // Supabaseの匿名キー
    supabase_anon_key: String,
}

// メソッド
impl UserRepository {
    // コンストラクタ
    pub fn new(client: Client, supabase_url: String, supabase_anon_key: String) -> Self {
        Self {
            // クライアント
            client,
            // SupabaseのURL
            supabase_url,
            // Supabaseの匿名キー
            supabase_anon_key,
        }
    }
}

// トレイト実装
#[async_trait]
impl UserRepositoryTrait for UserRepository {
    // 全件取得
    async fn find_all(&self) -> Result<Vec<User>, UserError> {
        let response = self.client
            .get(format!("{}/rest/v1/trans_users", self.supabase_url))
            .header("apikey", &self.supabase_anon_key)
            .send()
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(UserError::DatabaseError("Failed to retrieve user list.".to_string()));
        }

        let users: Vec<User> = response.json()
            .await
            .map_err(|e| UserError::JsonError(e.to_string()))?;

        Ok(users)
    }

    // 1件取得
    async fn find_by_id(&self, id: Uuid) -> Result<User, UserError> {
        let response = self.client
            .get(format!("{}/rest/v1/trans_users?id=eq.{}", self.supabase_url, id))
            .header("apikey", &self.supabase_anon_key)
            .send()
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(UserError::DatabaseError("User acquisition failed.".to_string()));
        }

        let users: Vec<User> = response.json()
            .await
            .map_err(|e| UserError::JsonError(e.to_string()))?;

        users.into_iter()
            .next()
            .ok_or(UserError::UserNotFound)
    }

    // 作成
    async fn create(&self, new_user: NewUser) -> Result<User, UserError> {
        // トランザクション開始
        let transaction_response = self.client
            .post(&format!("{}/rest/v1/rpc/begin_transaction", self.supabase_url))
            .header("apikey", &self.supabase_anon_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| UserError::DatabaseError(format!("Failed to start transaction: {}", e)))?;

        if !transaction_response.status().is_success() {
            return Err(UserError::DatabaseError("Failed to start transaction".to_string()));
        }

        // トランザクションIDを取得
        let transaction_data: serde_json::Value = transaction_response.json()
            .await
            .map_err(|e| UserError::DatabaseError(format!("Failed to parse transaction response: {}", e)))?;

        let transaction_id = transaction_data["transaction_id"]
            .as_str()
            .ok_or_else(|| UserError::DatabaseError("Failed to get transaction ID".to_string()))?;

        // パスワードのハッシュ化
        let hashed_password = hash(new_user.password.as_bytes(), DEFAULT_COST)
            .map_err(|e| UserError::DatabaseError(format!("Password hashing failed: {}", e)))?;

        let user = User {
            id: Uuid::new_v4(),
            username: new_user.username,
            email: new_user.email,
            password: hashed_password,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        let response = self.client
            .post(format!("{}/rest/v1/trans_users", self.supabase_url))
            .header("apikey", &self.supabase_anon_key)
            .header("Content-Type", "application/json")
            .header("Transaction-Id", transaction_id)
            .header("Prefer", "return=representation")
            .json(&user)
            .send()
            .await
            .map_err(|e| {
                // エラー時はロールバック
                let _ = self.client
                    .post(&format!("{}/rest/v1/rpc/rollback_transaction", self.supabase_url))
                    .header("apikey", &self.supabase_anon_key)
                    .json(&serde_json::json!({ "transaction_id": transaction_id }))
                    .send();
                UserError::DatabaseError(e.to_string())
            })?;

        match response.status() {
            status if status.is_success() => {
                // トランザクションをコミット
                let commit_response = self.client
                    .post(&format!("{}/rest/v1/rpc/commit_transaction", self.supabase_url))
                    .header("apikey", &self.supabase_anon_key)
                    .header("Content-Type", "application/json")
                    .json(&serde_json::json!({ "transaction_id": transaction_id }))
                    .send()
                    .await
                    .map_err(|e| UserError::DatabaseError(format!("Failed to commit transaction: {}", e)))?;

                if !commit_response.status().is_success() {
                    return Err(UserError::DatabaseError("Failed to commit transaction".to_string()));
                }

                let created_users: Vec<User> = response.json()
                    .await
                    .map_err(|e| UserError::JsonError(e.to_string()))?;
                
                Ok(created_users
                    .into_iter()
                    .next()
                    .ok_or(UserError::DatabaseError("User creation failed".to_string()))?)
            },
            status => {
                // エラー時はロールバック
                let _ = self.client
                    .post(&format!("{}/rest/v1/rpc/rollback_transaction", self.supabase_url))
                    .header("apikey", &self.supabase_anon_key)
                    .json(&serde_json::json!({ "transaction_id": transaction_id }))
                    .send()
                    .await;
                
                Err(UserError::DatabaseError(format!("User creation failed. Status: {}", status)))
            }
        }
    }

    // 更新
    async fn update(&self, id: Uuid, updated_user: NewUser) -> Result<User, UserError> {
        // まず、ユーザーが存在するか確認
        let check_response = self.client
            .get(format!("{}/rest/v1/trans_users?id=eq.{}", self.supabase_url, id))
            .header("apikey", &self.supabase_anon_key)
            .header("Prefer", "return=representation")
            .send()
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let users: Vec<User> = check_response
            .json()
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        if users.is_empty() {
            return Err(UserError::UserNotFound);
        }

        // トランザクション開始
        let transaction_response = self.client
            .post(&format!("{}/rest/v1/rpc/begin_transaction", self.supabase_url))
            .header("apikey", &self.supabase_anon_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| UserError::DatabaseError(format!("Failed to start transaction: {}", e)))?;

        if !transaction_response.status().is_success() {
            return Err(UserError::DatabaseError("Failed to start transaction".to_string()));
        }

        // パスワードのハッシュ化
        let hashed_password = hash(updated_user.password.as_bytes(), DEFAULT_COST)
            .map_err(|e| UserError::DatabaseError(format!("Password hashing failed: {}", e)))?;

        let update_data = serde_json::json!({
            "username": updated_user.username,
            "email": updated_user.email,
            "password": hashed_password,
            "updated_at": Utc::now().naive_utc()
        });

        let response = self.client
            .patch(&format!("{}/rest/v1/trans_users?id=eq.{}", self.supabase_url, id))
            .header("apikey", &self.supabase_anon_key)
            .header("Content-Type", "application/json")
            .header("Prefer", "tx=commit")
            .header("Prefer", "return=representation")
            .json(&update_data)
            .send()
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let status = response.status();
        let response_text = response.text().await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        match status {
            s if s.as_u16() == 200 => {
                let updated_user: Vec<User> = serde_json::from_str(&response_text)
                    .map_err(|e| UserError::JsonError(e.to_string()))?;
                
                Ok(updated_user
                    .into_iter()
                    .next()
                    .ok_or(UserError::UserNotFound)?)
            },
            _ => {
                // エラー時はロールバック
                let _ = self.client
                    .post(&format!("{}/rest/v1/rpc/rollback_transaction", self.supabase_url))
                    .header("apikey", &self.supabase_anon_key)
                    .send()
                    .await;
                
                Err(UserError::DatabaseError(format!("Failed to update user. Status: {}. Response: {}", status, response_text)))
            }
        }
    }

    // 削除
    async fn delete(&self, id: Uuid) -> Result<(), UserError> {
        // まず、ユーザーが存在するか確認
        let check_response = self.client
            .get(format!("{}/rest/v1/trans_users?id=eq.{}", self.supabase_url, id))
            .header("apikey", &self.supabase_anon_key)
            .header("Prefer", "return=representation")
            .send()
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let users: Vec<User> = check_response
            .json()
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        if users.is_empty() {
            return Err(UserError::UserNotFound);
        }

        // トランザクション開始
        let transaction_response = self.client
            .post(&format!("{}/rest/v1/rpc/begin_transaction", self.supabase_url))
            .header("apikey", &self.supabase_anon_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| UserError::DatabaseError(format!("Failed to start transaction: {}", e)))?;

        if !transaction_response.status().is_success() {
            return Err(UserError::DatabaseError("Failed to start transaction".to_string()));
        }

        let response = self.client
            .delete(&format!("{}/rest/v1/trans_users?id=eq.{}", self.supabase_url, id))
            .header("apikey", &self.supabase_anon_key)
            .header("Prefer", "tx=commit")
            .send()
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        if !response.status().is_success() {
            // エラー時はロールバック
            let _ = self.client
                .post(&format!("{}/rest/v1/rpc/rollback_transaction", self.supabase_url))
                .header("apikey", &self.supabase_anon_key)
                .send()
                .await;
            
            return Err(UserError::DatabaseError("Failed to delete user".to_string()));
        }

        Ok(())
    }
}
