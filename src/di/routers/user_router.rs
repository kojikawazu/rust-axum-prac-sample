use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::{Path, Json, State},
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;
// ハンドラー
use crate::di::handlers::user_handler::UserHandler;
// ユーザー
use crate::models::users::users::{NewUser};

// ルーター
pub struct UserRouter {
    handler: Arc<UserHandler>,
}

// メソッド
impl UserRouter {
    // コンストラクタ
    pub fn new(handler: Arc<UserHandler>) -> Self {
        Self { handler }
    }

    // ルート
    pub fn routes(&self) -> Router {
        let handler = Arc::clone(&self.handler);
        
        async fn get_users_handler(
            State(handler): State<Arc<UserHandler>>,
        ) -> impl IntoResponse {
            handler.get_users().await
        }

        async fn get_user_handler(
            State(handler): State<Arc<UserHandler>>,
            Path(id): Path<Uuid>,
        ) -> impl IntoResponse {
            handler.get_user(id).await
        }

        async fn create_user_handler(
            State(handler): State<Arc<UserHandler>>,
            Json(user): Json<NewUser>,
        ) -> impl IntoResponse {
            handler.create_user(Json(user)).await
        }

        async fn update_user_handler(
            State(handler): State<Arc<UserHandler>>,
            Path(id): Path<Uuid>,
            Json(user): Json<NewUser>,
        ) -> impl IntoResponse {
            handler.update_user(id, Json(user)).await
        }

        async fn delete_user_handler(
            State(handler): State<Arc<UserHandler>>,
            Path(id): Path<Uuid>,
        ) -> impl IntoResponse {
            handler.delete_user(id).await
        }

        Router::new()
            .route("/di/users", get(get_users_handler))
            .route("/di/users/:id", get(get_user_handler))
            .route("/di/users", post(create_user_handler))
            .route("/di/users/:id", put(update_user_handler))
            .route("/di/users/:id", delete(delete_user_handler))
            .with_state(handler)
    }
}