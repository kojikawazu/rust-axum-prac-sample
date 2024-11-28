// 必要なクレートのインポート
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use bcrypt::{hash, DEFAULT_COST};
// アプリケーションの状態のインポート
use crate::state::app_state::AppState;
// ユーザーモデルのインポート
use crate::models::users::users::{User, NewUser};
// ユーザーエラーのインポート
use crate::errors::users::user_error::UserError;

// レスポンスの詳細なデバッグ情報を出力
//println!("Create response status: {:?}", response.status());
//println!("Create response headers: {:?}", response.headers());

// ユーザーの一覧を取得する関数
#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "ユーザー一覧を取得成功", body = Vec<User>),
        (status = 500, description = "サーバーエラー", body = String)
    ),
    tag = "users"
)]
pub async fn get_users(
    State(state): State<Arc<AppState>>
) -> Result<Json<Vec<User>>, (StatusCode, String)>  {
    // Supabaseからユーザーの一覧を取得
    let response = state
        .client
        .get(format!("{}/rest/v1/trans_users", state.supabase_url))
        .header("apikey", &state.supabase_anon_key)
        .send()
        .await
        .map_err(|e| UserError::DatabaseError(e.to_string()))?;

    // ステータスコードの確認
    if !response.status().is_success() {
        return Err(UserError::DatabaseError("Failed to retrieve user list.".to_string()).into());
    }

    // ユーザーデータのパース
    let users: Vec<User> = response
        .json()
        .await
        .map_err(|e| UserError::InvalidData(e.to_string()))?;

    // JSONデータを返す
    Ok(Json(users))
}

// 特定のユーザーを取得する関数
#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = crate::models::UuidWrapper, Path, description = "ユーザーID")
    ),
    responses(
        (status = 200, description = "ユーザー取得成功", body = User),
        (status = 404, description = "ユーザーが見つかりません", body = String),
        (status = 500, description = "サーバーエラー", body = String)
    ),
    tag = "users"
)]
pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>
) -> Result<Json<User>, (StatusCode, String)> {
    // Supabaseから特定のユーザーを取得
    let response = state
        .client
        .get(format!("{}/rest/v1/trans_users?id=eq.{}", state.supabase_url, id))
        .header("apikey", &state.supabase_anon_key)
        .send()
        .await
        .map_err(|e| UserError::DatabaseError(e.to_string()))?;

    // ステータスコードの確認
    if !response.status().is_success() {
        return Err(UserError::DatabaseError("User acquisition failed.".to_string()).into());
    }

    // ユーザーデータのパース
    let users: Vec<User> = response
        .json()
        .await
        .map_err(|e| UserError::InvalidData(e.to_string()))?;

    // ユーザーが存在するか確認
    let user = users
        .into_iter()
        .next()
        .ok_or(UserError::UserNotFound)?;

    // JSONデータを返す
    Ok(Json(user))
}

// 新しいユーザーを作成する関数
#[utoipa::path(
    post,
    path = "/users",
    request_body = NewUser,
    responses(
        (status = 201, description = "ユーザー作成成功", body = User),
        (status = 400, description = "無効なリクエストデータ", body = String),
        (status = 500, description = "サーバーエラー", body = String)
    ),
    tag = "users"
)]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(new_user): Json<NewUser>
) -> Result<Json<User>, (StatusCode, String)> {
    // バリデーションチェック
    if new_user.username.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Username cannot be empty".to_string()));
    }

    if !new_user.email.contains('@') {
        return Err((StatusCode::BAD_REQUEST, "Invalid email format".to_string()));
    }

    if new_user.password.len() < 8 {
        return Err((StatusCode::BAD_REQUEST, "Password must be at least 8 characters".to_string()));
    }

    // トランザクション開始
    let transaction_response = state
        .client
        .post(&format!("{}/rest/v1/rpc/begin_transaction", state.supabase_url))
        .header("apikey", &state.supabase_anon_key)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({}))
        .send()
        .await
        .map_err(|e| UserError::DatabaseError(format!("Failed to start transaction: {}", e)))?;

    // トランザクション開始のレスポンスを処理
    if !transaction_response.status().is_success() {
        let error_text = transaction_response.text().await
            .unwrap_or_else(|_| "Unknown error".to_string());
        println!("Transaction start error: {}", error_text);
        return Err(UserError::DatabaseError("Failed to start transaction".to_string()).into());
    }

    // 空のレスポンスボディは無視
    let _ = transaction_response.text().await;

    // パスワードをハッシュ化
    let hashed_password = hash(new_user.password.as_bytes(), DEFAULT_COST)
        .map_err(|e| UserError::DatabaseError(format!("Password hashing failed: {}", e)))?;

    // 新しいユーザーのデータを作成
    let user = User {
        id: Uuid::new_v4(),
        username: new_user.username,
        email: new_user.email,
        password: hashed_password,
        created_at: Utc::now().naive_utc(), 
        updated_at: Utc::now().naive_utc(),
    };

    // Supabaseに新しいユーザーを作成
    let response = state
        .client
        .post(format!("{}/rest/v1/trans_users", state.supabase_url))
        .header("apikey", &state.supabase_anon_key)
        .header("Content-Type", "application/json")
        .header("Prefer", "tx=commit")
        .header("Prefer", "return=representation")
        .json(&user)
        .send()
        .await
        .map_err(|e| {
            // エラー時はトランザクションをロールバック
            let _ = state
                .client
                .post(&format!("{}/rest/v1/rpc/rollback_transaction", state.supabase_url))
                .header("apikey", &state.supabase_anon_key)
                .send();
            UserError::DatabaseError(e.to_string())
        })?;

    // レスポンスの処理
    match response.status() {
        status if status.is_success() => {
            // 作成されたユーザー情報をパース
            let created_users: Vec<User> = response
                .json()
                .await
                .map_err(|e| UserError::InvalidData(e.to_string()))?;
            
            // 作成されたユーザー情報を返す
            Ok(Json(created_users
                .into_iter()
                .next()
                .ok_or(UserError::DatabaseError("User creation failed".to_string()))?))
        },
        status => {
            // エラー時はトランザクションをロールバック
            let _ = state
                .client
                .post(&format!("{}/rest/v1/rpc/rollback_transaction", state.supabase_url))
                .header("apikey", &state.supabase_anon_key)
                .send()
                .await;
            
            Err((
                StatusCode::BAD_REQUEST,
                format!("User creation failed. Status: {}", status)
            ))
        }
    }
}

// ユーザーを更新する関数
#[utoipa::path(
    patch,
    path = "/users/{id}",
    params(
        ("id" = crate::models::UuidWrapper, Path, description = "更新対象のユーザーID")
    ),
    request_body = NewUser,
    responses(
        (status = 200, description = "ユーザー更新成功", body = User),
        (status = 404, description = "ユーザーが見つかりません", body = String),
        (status = 400, description = "無効なリクエストデータ", body = String),
        (status = 500, description = "サーバーエラー", body = String)
    ),
    tag = "users"
)]
pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(updated_user): Json<NewUser>
) -> Result<Json<User>, (StatusCode, String)> {
    // まず、ユーザーが存在するか確認
    let check_response = state
        .client
        .get(format!("{}/rest/v1/trans_users?id=eq.{}", state.supabase_url, id))
        .header("apikey", &state.supabase_anon_key)
        .header("Prefer", "return=representation")
        .send()
        .await
        .map_err(|e| UserError::DatabaseError(e.to_string()))?;

    // レスポンスボディを取得して、ユーザーの存在を確認
    let users: Vec<User> = check_response
        .json()
        .await
        .map_err(|e| UserError::DatabaseError(e.to_string()))?;

    // ユーザーが存在しない場合はエラーを返す
    if users.is_empty() {
        return Err(UserError::UserNotFound.into());
    }

    // トランザクション開始
    let transaction_response = state
        .client
        .post(&format!("{}/rest/v1/rpc/begin_transaction", state.supabase_url))
        .header("apikey", &state.supabase_anon_key)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({}))
        .send()
        .await
        .map_err(|e| UserError::DatabaseError(format!("Failed to start transaction: {}", e)))?;

    // トランザクション開始のレスポンスを処理
    if !transaction_response.status().is_success() {
        let error_text = transaction_response.text().await
            .unwrap_or_else(|_| "Unknown error".to_string());
        println!("Transaction start error: {}", error_text);
        return Err(UserError::DatabaseError("Failed to start transaction".to_string()).into());
    }

    // 空のレスポンスボディは無視
    let _ = transaction_response.text().await;

    // パスワードをハッシュ化
    let hashed_password = hash(updated_user.password.as_bytes(), DEFAULT_COST)
        .map_err(|e| UserError::DatabaseError(format!("Password hashing failed: {}", e)))?;
    // 更新データの準備
    let update_data = serde_json::json!({
        "username": updated_user.username,
        "email": updated_user.email,
        "password": hashed_password,
        "updated_at": Utc::now().naive_utc()
    });
    
    // トランザクション内でユーザーを更新
    let response = state
        .client
        .patch(&format!("{}/rest/v1/trans_users?id=eq.{}", state.supabase_url, id))
        .header("apikey", &state.supabase_anon_key)
        .header("Content-Type", "application/json")
        .header("Prefer", "tx=commit")  // トランザクションをコミット
        .header("Prefer", "return=representation")
        .json(&update_data)
        .send()
        .await
        .map_err(|e| {
            // エラー時はトランザクションをロールバック
            let _ = state
                .client
                .post(&format!("{}/rest/v1/rpc/rollback_transaction", state.supabase_url))
                .header("apikey", &state.supabase_anon_key)
                .header("Authorization", format!("Bearer {}", state.supabase_anon_key))
                .send();
            UserError::DatabaseError(e.to_string())
        })?;

    // ステータスコードを先に保存
    let status = response.status();
    // レスポンスボディを取得
    let response_text = response.text().await
        .map_err(|e| UserError::DatabaseError(e.to_string()))?;

    // ステータスコードに応じた処理
    match status {
        s if s.as_u16() == 200 => {
            // レスポンスボディをUserオブジェクトとしてパース
            let updated_user: Vec<User> = serde_json::from_str(&response_text)
                .map_err(|e| UserError::InvalidData(e.to_string()))?;
            
            // 更新されたユーザー情報を返す
            Ok(Json(updated_user
                .into_iter()
                .next()
                .ok_or(UserError::UserNotFound)?))
        },
        s if s.as_u16() == 404 => {
            // エラー時はトランザクションをロールバック
            let _ = state
                .client
                .post(&format!("{}/rest/v1/rpc/rollback_transaction", state.supabase_url))
                .header("apikey", &state.supabase_anon_key)
                .send()
                .await;
            Err(UserError::UserNotFound.into())
        },
        _ => {
            // エラー時はトランザクションをロールバック
            let _ = state
                .client
                .post(&format!("{}/rest/v1/rpc/rollback_transaction", state.supabase_url))
                .header("apikey", &state.supabase_anon_key)
                .send()
                .await;
            Err(UserError::DatabaseError(format!("Failed to update user. Status: {}. Response: {}", status, response_text)).into())
        }
    }
}

// ユーザーを削除する関数
#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id" = crate::models::UuidWrapper, Path, description = "削除対象のユーザーID")
    ),
    responses(
        (status = 200, description = "ユーザー削除成功", body = String),
        (status = 404, description = "ユーザーが見つかりません", body = String),
        (status = 500, description = "サーバーエラー", body = String)
    ),
    tag = "users"
)]
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>
) -> Result<Json<String>, (StatusCode, String)> {
    // まず、ユーザーが存在するか確認
    let check_response = state
        .client
        .get(format!("{}/rest/v1/trans_users?id=eq.{}", state.supabase_url, id))
        .header("apikey", &state.supabase_anon_key)
        .header("Prefer", "return=representation")
        .send()
        .await
        .map_err(|e| UserError::DatabaseError(e.to_string()))?;

    // レスポンスボディを取得して、ユーザーの存在を確認
    let users: Vec<User> = check_response
        .json()
        .await
        .map_err(|e| UserError::DatabaseError(e.to_string()))?;

    // ユーザーが存在しない場合はエラーを返す
    if users.is_empty() {
        return Err(UserError::UserNotFound.into());
    }

    // トランザクション開始
    let transaction_response = state
        .client
        .post(&format!("{}/rest/v1/rpc/begin_transaction", state.supabase_url))
        .header("apikey", &state.supabase_anon_key)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({}))
        .send()
        .await
        .map_err(|e| UserError::DatabaseError(format!("Failed to start transaction: {}", e)))?;

    // トランザクション開始のレスポンスを処理
    if !transaction_response.status().is_success() {
        let error_text = transaction_response.text().await
            .unwrap_or_else(|_| "Unknown error".to_string());
        println!("Transaction start error: {}", error_text);
        return Err(UserError::DatabaseError("Failed to start transaction".to_string()).into());
    }

    // 空のレスポンスボディは無視
    let _ = transaction_response.text().await;

    // ユーザーを削除
    let response = state
        .client
        .delete(format!("{}/rest/v1/trans_users?id=eq.{}", state.supabase_url, id))
        .header("apikey", &state.supabase_anon_key)
        .header("Prefer", "tx=commit")
        // レスポンスにデータを含める
        .header("Prefer", "return=representation")
        .send()
        .await
        .map_err(|e| {
            // エラー時はトランザクションをロールバック
            let _ = state
                .client
                .post(&format!("{}/rest/v1/rpc/rollback_transaction", state.supabase_url))
                .header("apikey", &state.supabase_anon_key)
                .header("Authorization", format!("Bearer {}", state.supabase_anon_key))
                .send();
            UserError::DatabaseError(e.to_string())
        })?;

    // 削除結果を確認
    let deleted_users: Vec<User> = response
        .json()
        .await
        .map_err(|e| UserError::DatabaseError(e.to_string()))?;

    // 削除されたユーザーが存在しない場合はエラー
    if deleted_users.is_empty() {
        let _ = state
            .client
            .post(&format!("{}/rest/v1/rpc/rollback_transaction", state.supabase_url))
            .header("apikey", &state.supabase_anon_key)
            .send()
            .await;
        return Err(UserError::UserNotFound.into());
    }

    // 成功メッセージを返す
    Ok(Json("User deleted successfully".to_string()))
}