// モジュールのインポート
mod routes;
mod services;
mod models;
mod state;
mod errors;
// 必要なクレートのインポート
use axum::serve;
use dotenv::dotenv;
use std::{env, sync::Arc};
use std::net::SocketAddr;
use tokio::net::TcpListener;
// アプリケーションの状態のインポート
use crate::state::AppState;
// ルーティングのインポート
use crate::routes::create_routes;

// メイン関数
#[tokio::main]
async fn main() {
    // 環境変数を読み込む
    dotenv().ok();

    // SupabaseのURLを環境変数から取得
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_anon_key = env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY must be set");
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    // アプリケーションの状態を作成
    let state = Arc::new(AppState::new(supabase_url, supabase_anon_key));
    // ルーティングを作成
    let app = create_routes(state);
    // ソケットアドレスを作成
    let addr = format!("127.0.0.1:{}", port).parse::<SocketAddr>().unwrap();

    println!("Server running at http://127.0.0.1:{}", port);

    // ソケットアドレスをバインドして起動
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}