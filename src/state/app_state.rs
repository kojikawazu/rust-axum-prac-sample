// 必要なクレートのインポート
use reqwest::Client;

// アプリケーションの状態を管理する構造体
#[derive(Clone)]
pub struct AppState {
    // SupabaseのURL
    pub supabase_url: String,
    // Supabaseの匿名キー
    pub supabase_anon_key: String,
    // HTTPクライアント
    pub client: Client,
    // JWTシークレット
    pub jwt_secret: String,
}

// AppStateの実装
impl AppState {
    // AppStateの新しいインスタンスを作成する関数
    pub fn new(supabase_url: String, supabase_anon_key: String) -> Self {
        // 新しいAppStateインスタンスを作成
        Self {
            // SupabaseのURLを設定
            supabase_url,
            // Supabaseの匿名キーを設定
            supabase_anon_key,
            // HTTPクライアントを作成
            client: Client::new(),
            // JWTシークレットを設定
            jwt_secret: String::new(),
        }
    }
}
