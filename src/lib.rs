// モジュールの宣言と公開
pub mod di;
pub mod errors;
pub mod models;
pub mod routes;
pub mod services;
pub mod state;
// 必要なクレートのインポート
use axum::Router;
use std::sync::Arc;
// アプリケーションの状態のインポート
use crate::state::AppState;

// アプリケーションの作成
pub async fn create_app() -> Router {
    // 環境変数から値を取得
    let supabase_url = std::env::var("SUPABASE_URL")
        .expect("SUPABASE_URL must be set");
    let supabase_anon_key = std::env::var("SUPABASE_ANON_KEY")
        .expect("SUPABASE_ANON_KEY must be set");

    // AppStateの初期化
    let state = Arc::new(AppState::new(
        supabase_url,
        supabase_anon_key,
    ));
    
    // ルーターを作成
    routes::create_routes(state)
}
