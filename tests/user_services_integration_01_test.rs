// 必要なクレートのインポート
use backend::{
    create_app,
    models::users::users::{User, NewUser},
};
// 必要なクレートのインポート
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::util::ServiceExt;
use axum::body::Bytes; 
use serde_json::from_slice;
use dotenv::dotenv;
use futures_util::StreamExt;

// ユーザーのCRUD操作をテストする関数
#[tokio::test]
async fn test_user_crud_operations() {
    // .envファイルを読み込む
    dotenv().ok();
    // テスト用のアプリケーションを作成
    let app = create_app().await;

    // ------------------------------------------------------------------------
    // 1. CREATE: 新しいユーザーを作成
    // ------------------------------------------------------------------------
    let new_user = NewUser {
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    // ユーザーを作成
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&new_user).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードがOKであることを確認
    assert_eq!(response.status(), StatusCode::OK);

    // 作成されたユーザー情報を取得
    let body = response.into_body();
    let bytes = body_to_bytes(body).await;
    let created_user: User = from_slice(&bytes).unwrap();

    // ------------------------------------------------------------------------
    // 2. READ ALL: 全ユーザーを取得
    // ------------------------------------------------------------------------
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/users")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードがOKであることを確認
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = body_to_bytes(response.into_body()).await;
    let users: Vec<User> = serde_json::from_slice(&bytes).unwrap();
    assert!(!users.is_empty());

    // ------------------------------------------------------------------------
    // 3. READ ONE: 特定のユーザーを取得
    // ------------------------------------------------------------------------
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&format!("/users/{}", created_user.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードがOKであることを確認
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = body_to_bytes(response.into_body()).await;
    let fetched_user: User = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(fetched_user.id, created_user.id);

    // ------------------------------------------------------------------------
    // 4. UPDATE: ユーザー情報を更新
    // ------------------------------------------------------------------------
    let updated_user = NewUser {
        username: "updated_user".to_string(),
        email: "updated@example.com".to_string(),
        password: "newpassword123".to_string(),
    };

    // ユーザーを更新
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(&format!("/users/{}", created_user.id))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&updated_user).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードがOKであることを確認
    assert_eq!(response.status(), StatusCode::OK);

    // 更新されたことを確認
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&format!("/users/{}", created_user.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードがOKであることを確認
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = body_to_bytes(response.into_body()).await;
    let updated_fetched_user: User = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(updated_fetched_user.username, updated_user.username);
    assert_eq!(updated_fetched_user.email, updated_user.email);

    // ------------------------------------------------------------------------
    // 5. DELETE: ユーザーを削除
    // ------------------------------------------------------------------------
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(&format!("/users/{}", created_user.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードがOKであることを確認
    assert_eq!(response.status(), StatusCode::OK);

    // 削除されたことを確認
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&format!("/users/{}", created_user.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードがNOT_FOUNDであることを確認
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ボディをバイト列に変換するヘルパー関数
async fn body_to_bytes(body: Body) -> Bytes {
    let mut bytes = Vec::new();
    let mut stream = body.into_data_stream();
    
    while let Some(chunk) = stream.next().await {
        if let Ok(data) = chunk {
            bytes.extend_from_slice(&data);
        }
    }
    
    Bytes::from(bytes)
}