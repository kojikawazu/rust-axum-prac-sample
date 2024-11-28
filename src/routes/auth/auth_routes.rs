// 必要なクレートのインポート
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
// アプリケーションの状態のインポート
use crate::state::app_state::AppState;
// 認証サービスのインポート
use crate::services::auth::auth_services::{sign_in, sign_out, check_auth};

// 認証ルーティングを作成する関数
pub fn auth_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/signin", post(sign_in))
        .route("/auth/signout", post(sign_out))
        .route("/auth/check", get(check_auth))
        .with_state(app_state)
}