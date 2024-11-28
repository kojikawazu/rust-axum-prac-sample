// ルーティングのモジュールの宣言
pub mod users;
pub mod auth;
// 必要なクレートのインポート
use axum::Router;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
// アプリケーションの状態のインポート
use crate::state::app_state::AppState;

// APIドキュメントの定義
#[derive(OpenApi)]
#[openapi(
    // パスの定義
    paths(
        // ユーザー関連のエンドポイント
        crate::services::users::user_services::get_users,
        crate::services::users::user_services::get_user_by_id,
        crate::services::users::user_services::create_user,
        crate::services::users::user_services::update_user,
        crate::services::users::user_services::delete_user,
        // 認証関連のエンドポイント
        crate::services::auth::auth_services::sign_in,
        crate::services::auth::auth_services::check_auth,
        crate::services::auth::auth_services::sign_out,
    ),
    // モデルのスキーマの定義
    components(
        schemas(
            // ユーザーモデル
            crate::models::users::users::User,
            crate::models::users::users::NewUser,
            // 認証モデル
            crate::models::auth::auth::SignInCredentials,
            crate::models::auth::auth::Claims,
            crate::models::auth::auth::AuthResponse,
            // 基本型のスキーマラッパー
            crate::models::NaiveDateTimeWrapper,
            crate::models::UuidWrapper
        ),
    ),
    // セキュリティスキーマの追加
    modifiers(&SecurityAddon),
    // タグの定義
    tags(
        (name = "users", description = "ユーザー管理API"),
        (name = "auth", description = "認証API")
    )
)]

// APIドキュメントの定義
struct ApiDoc;
// セキュリティスキーマの定義
struct SecurityAddon;

// Modifyトレイトの実装
impl utoipa::Modify for SecurityAddon {
    // Modifyトレイトのメソッドの実装
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // コンポーネントが存在する場合
        if let Some(components) = openapi.components.as_mut() {
            // セキュリティスキームを追加
            components.add_security_scheme(
                // セキュリティスキームの名前
                "bearer_auth",
                // HTTPセキュリティスキームの定義
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        // Bearer認証スキームの定義
                        utoipa::openapi::security::HttpAuthScheme::Bearer
                    )
                ),
            )
        }
    }
}

// ルーティングを作成する関数
pub fn create_routes(state: Arc<AppState>) -> Router {
    // 新しいルーターを作成し、ユーザールーティングをマージ
    Router::new()
        // ユーザールーティングをマージ
        .merge(users::user_routes::user_routes(state.clone()))
        // 認証ルーティングをマージ
        .merge(auth::auth_routes::auth_routes(state))
        // Swagger UIをマージ
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
}