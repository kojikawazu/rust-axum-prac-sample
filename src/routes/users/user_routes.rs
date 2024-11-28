// 必要なクレートのインポート
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
// アプリケーションの状態のインポート
use crate::state::app_state::AppState;
// ユーザーサービスのインポート
use crate::services::users::user_services;

// ユーザールーティングを作成する関数
pub fn user_routes(state: Arc<AppState>) -> Router {
    // 新しいルーターを作成し、ユーザールーティングを設定
    Router::new()
        // ユーザーの一覧を取得するルートを設定 
        .route("/users", get(user_services::get_users))
        // ユーザーを作成するルートを設定
        .route("/users", post(user_services::create_user))
        // 特定のユーザーを取得するルートを設定
        .route("/users/:id", get(user_services::get_user_by_id))
        // ユーザーを更新するルートを設定
        .route("/users/:id", put(user_services::update_user))
        // ユーザーを削除するルートを設定
        .route("/users/:id", delete(user_services::delete_user))
        // アプリケーションの状態をルーターに渡す
        .with_state(state)
}