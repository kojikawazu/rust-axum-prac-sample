// 必要なクレートのインポート
use std::sync::Arc;
use reqwest::Client;
use crate::di::repositories::user_repository::{UserRepositoryTrait, UserRepository};
use crate::di::services::user_di_service::UserDIService;
use crate::di::handlers::user_handler::UserHandler;
use crate::di::routers::user_router::UserRouter;

// コンテナ
#[allow(dead_code)]
pub struct Container {
    user_repository: Arc<dyn UserRepositoryTrait>,
    user_service: Arc<UserDIService>,
    user_handler: Arc<UserHandler>,
    user_router: Arc<UserRouter>,
}

// メソッド
impl Container {
    // コンストラクタ
    pub fn new(
        client: Client,
        supabase_url: String,
        supabase_anon_key: String,
    ) -> Self {
        // リポジトリの初期化
        let user_repository = Arc::new(UserRepository::new(
            client,
            supabase_url,
            supabase_anon_key,
        ));
        
        // サービスの初期化
        let user_service = Arc::new(UserDIService::new(user_repository.clone()));
        
        // ハンドラーの初期化
        let user_handler = Arc::new(UserHandler::new(user_service.clone()));
        
        // ルーターの初期化
        let user_router = Arc::new(UserRouter::new(user_handler.clone()));

        Self {
            user_repository,
            user_service,
            user_handler,
            user_router,
        }
    }

    // ルーター
    pub fn user_router(&self) -> Arc<UserRouter> {
        self.user_router.clone()
    }
}