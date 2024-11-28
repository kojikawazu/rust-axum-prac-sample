// 必要なクレートのインポート
use backend::{
    create_app,
    models::users::users::NewUser,
};
// 必要なクレートのインポート
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::util::ServiceExt;
use dotenv::dotenv;

// 不正なユーザー操作のテスト
#[tokio::test]
async fn test_invalid_user_operations() {
    // .envファイルを読み込む
    dotenv().ok();
    
    // テスト用のアプリケーションを作成
    let app = create_app().await;

    // ------------------------------------------------------------------------
    // 1. 不正なユーザーデータでの作成テスト
    // ------------------------------------------------------------------------
    let invalid_user = NewUser {
        username: "".to_string(),  // 空の名前
        email: "invalid-email".to_string(),  // 不正なメールアドレス
        password: "123".to_string(),  // 短すぎるパスワード
    };

    // ユーザーを作成
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&invalid_user).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードがBAD_REQUESTであることを確認
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // ------------------------------------------------------------------------
    // 2. 存在しないユーザーの取得テスト
    // ------------------------------------------------------------------------
    let response = app.clone()
    .oneshot(
        Request::builder()
            .method("GET")
            // 有効なUUID形式を使用
            .uri("/users/00000000-0000-0000-0000-000000000000") 
            .body(Body::empty())
            .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // ------------------------------------------------------------------------
    // 3. 不正なIDでのアクセステスト
    // ------------------------------------------------------------------------
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/users/invalid_id")  // 数値ではないID
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードがBAD_REQUESTであることを確認
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // ------------------------------------------------------------------------
    // 4. 不正なJSONデータでの作成テスト
    // ------------------------------------------------------------------------
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("Content-Type", "application/json")
                .body(Body::from("{invalid_json}"))  // 不正なJSON
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードがBAD_REQUESTであることを確認
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // ------------------------------------------------------------------------
    // 5. 存在しないユーザーの更新テスト
    // ------------------------------------------------------------------------
    let update_user = NewUser {
        username: "updated".to_string(),
        email: "updated@example.com".to_string(),
        password: "password123".to_string(),
    };

    // ユーザーを更新
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/users/00000000-0000-0000-0000-000000000000")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&update_user).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードがNOT_FOUNDであることを確認
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // ------------------------------------------------------------------------
    // 6. 存在しないユーザーの削除テスト
    // ------------------------------------------------------------------------
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/users/00000000-0000-0000-0000-000000000000")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードがNOT_FOUNDであることを確認
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}