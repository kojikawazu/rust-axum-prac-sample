// 必要なクレートのインポート
use axum::{
    extract::{State, Json},
    http::{StatusCode, HeaderMap},
};
use std::sync::Arc;
use bcrypt::verify;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};

// アプリケーションの状態のインポート
use crate::state::app_state::AppState;
// 認証モデルのインポート
use crate::models::auth::auth::{SignInCredentials, Claims, AuthResponse};
// ユーザーモデル
use crate::models::users::users::User;
// 認証エラーのインポート
use crate::errors::auth::auth_error::AuthError;

// サインイン処理
#[utoipa::path(
    post,
    path = "/auth/sign-in",
    request_body = SignInCredentials,
    responses(
        (status = 200, description = "サインイン成功", body = AuthResponse),
        (status = 401, description = "認証失敗"),
        (status = 500, description = "サーバーエラー")
    ),
    tag = "auth"
)]
pub async fn sign_in(
    State(state): State<Arc<AppState>>,
    Json(credentials): Json<SignInCredentials>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    // メールアドレスでユーザーを検索
    let response = state
        .client
        .get(format!("{}/rest/v1/trans_users?email=eq.{}", state.supabase_url, credentials.email))
        .header("apikey", &state.supabase_anon_key)
        .send()
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

    let users: Vec<User> = response
        .json()
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

    let user = users
        .into_iter()
        .next()
        .ok_or(AuthError::InvalidCredentials)?;

    // パスワードの検証
    if !verify(&credentials.password, &user.password)
        .map_err(|e| AuthError::DatabaseError(e.to_string()))? {
        return Err(AuthError::InvalidCredentials.into());
    }

    // JWTトークンの生成
    let claims = Claims::new(&user.id);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|e| AuthError::TokenCreationError(e.to_string()))?;

    Ok(Json(AuthResponse { token, user }))
}

// 認証状態チェック
#[utoipa::path(
    get,
    path = "/auth/check",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "認証成功", body = User),
        (status = 401, description = "認証失敗"),
        (status = 500, description = "サーバーエラー")
    ),
    tag = "auth"
)]
pub async fn check_auth(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<User>, (StatusCode, String)> {
    // Authorizationヘッダーからトークンを取得
    let token = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(AuthError::InvalidToken)?;

    // トークンの検証
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AuthError::InvalidToken)?;

    // ユーザー情報の取得
    let response = state
        .client
        .get(format!("{}/rest/v1/trans_users?id=eq.{}", state.supabase_url, claims.claims.sub))
        .header("apikey", &state.supabase_anon_key)
        .send()
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

    let users: Vec<User> = response
        .json()
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

    let user = users
        .into_iter()
        .next()
        .ok_or(AuthError::UserNotFound)?;

    Ok(Json(user))
}

// サインアウト
#[utoipa::path(
    post,
    path = "/auth/sign-out",
    responses(
        (status = 200, description = "サインアウト成功", body = String),
        (status = 500, description = "サーバーエラー")
    ),
    tag = "auth"
)]
pub async fn sign_out() -> Result<Json<String>, (StatusCode, String)> {
    Ok(Json("Successfully signed out".to_string()))
}