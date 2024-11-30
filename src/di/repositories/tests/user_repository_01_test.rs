use std::time::Duration;
use reqwest::Client;
use serde_json::json;
use chrono::Utc;
use uuid::Uuid;
// リポジトリのインポート
use crate::di::repositories::user_repository::{UserRepository, UserRepositoryTrait};
// エラーのインポート
use crate::errors::users::user_error::UserError;
// モデルのインポート
use crate::models::users::users::{User, NewUser};
// ヘルパーのインポート
use super::helpers::{
    create_test_user,
    create_test_user_with_id,
    setup_mocks,
    setup_update_mocks,
    setup_delete_mocks,
    MockHandles
};

// find_allのテスト
#[tokio::test]
async fn test_find_all() {
    // モックサーバーの設定
    let mut mock_server = mockito::Server::new_async().await;
    // モックサーバーのURLを取得
    let mock_url = mock_server.url();

    // テストデータの準備
    let test_user = create_test_user_with_id();
    // テストデータをJSONに変換
    let response_data = json!([test_user]);

    // モックレスポンスの設定
    let mock = mock_server
        .mock("GET", "/rest/v1/trans_users")
        .with_header("apikey", "test_key")
        .with_status(200)
        .with_body(response_data.to_string())
        .create_async()
        .await;

    // HTTPクライアントの初期化
    let client = Client::new();
    // リポジトリの初期化
    let repository = UserRepository::new(
        client,
        mock_url,
        "test_key".to_string(),
    );

    // find_allのテスト
    let result = repository.find_all().await;
    // 結果の確認
    assert!(result.is_ok());
    // ユーザーの取得
    let users = result.unwrap();
    // ユーザーの数とユーザーの内容の確認
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].username, test_user.username);
    assert_eq!(users[0].email, test_user.email);

    // モックの確認
    mock.assert_async().await;
}

// find_by_idのテスト
#[tokio::test]
async fn test_find_by_id() {
    // モックサーバーの設定
    let mut mock_server = mockito::Server::new_async().await;
    // モックサーバーのURLを取得
    let mock_url = mock_server.url();

    // テストデータの準備
    let test_user = create_test_user_with_id();
    // ユーザーのIDを取得
    let user_id = test_user.id;
    // テストデータをJSONに変換
    let response_data = json!([test_user]);

    // モックレスポンスの設定
    let path = format!("/rest/v1/trans_users?id=eq.{}", user_id);
    // モックレスポンスの設定
    let mock = mock_server
        .mock("GET", path.as_str())
        .with_header("apikey", "test_key")
        .with_status(200)
        .with_body(response_data.to_string())
        .create_async()
        .await;

    // HTTPクライアントの初期化
    let client = Client::new();
    // リポジトリの初期化
    let repository = UserRepository::new(
        client,
        mock_url,
        "test_key".to_string(),
    );

    // find_by_idのテスト
    let result = repository.find_by_id(user_id).await;
    // 結果の確認
    assert!(result.is_ok());
    // ユーザーの取得
    let user = result.unwrap();
    // ユーザーのID、ユーザー名、メールアドレスの確認
    assert_eq!(user.id, user_id);
    assert_eq!(user.username, test_user.username);
    assert_eq!(user.email, test_user.email);

    // モックの確認
    mock.assert_async().await;
}

#[tokio::test]
async fn test_create() {
    let mut mock_server = mockito::Server::new_async().await;
    let mock_url = mock_server.url();

    // テストデータの準備
    let new_user = create_test_user();
    let created_user = create_test_user_with_id();
    
    println!("Setting up mocks for URL: {}", mock_url);
    println!("Test user data: {:?}", new_user);

    // 各モックの作成
    let mocks = setup_mocks(&mut mock_server, &created_user).await;
    
    let client = Client::new();
    let repository = UserRepository::new(
        client,
        mock_url.clone(),
        "test_key".to_string(),
    );
   
    println!("Executing create operation...");
    let result = repository.create(new_user.clone()).await;
   
    // 結果の検証
    verify_add_result(&result, &new_user, &mocks).await;
}

#[tokio::test]
async fn test_update() {
    let mut mock_server = mockito::Server::new_async().await;
    let mock_url = mock_server.url();

    let user_id = Uuid::new_v4();
    let update_user = create_test_user();
    let updated_user = User {
        id: user_id,
        username: update_user.username.clone(),
        email: update_user.email.clone(),
        password: "hashed_new_password".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    println!("Setting up mocks for URL: {}", mock_url);
    let mocks = setup_update_mocks(&mut mock_server, &user_id, &updated_user).await;

    let client = Client::new();
    let repository = UserRepository::new(
        client,
        mock_url,
        "test_key".to_string(),
    );

    println!("Executing update operation...");
    let result = repository.update(user_id, update_user.clone()).await;
    
    verify_update_result(&result, user_id, &update_user, &mocks).await;
}

#[tokio::test]
async fn test_delete() {
    let mut mock_server = mockito::Server::new_async().await;
    let mock_url = mock_server.url();

    let user_id = Uuid::new_v4();

    println!("Setting up mocks for URL: {}", mock_url);
    let mocks = setup_delete_mocks(&mut mock_server, &user_id).await;

    let client = Client::new();
    let repository = UserRepository::new(
        client,
        mock_url,
        "test_key".to_string(),
    );

    println!("Executing delete operation...");
    let result = repository.delete(user_id).await;
    
    verify_delete_result(&result, &mocks).await;
}

// 追加結果の検証
async fn verify_add_result(result: &Result<User, UserError>, new_user: &NewUser, mocks: &MockHandles) {
    // 結果のログ
    match result {
        Ok(user) => println!("Create succeeded: {:?}", user),
        Err(e) => println!("Create failed with error: {:?}", e),
    }
   
    // モックの待機
    println!("Waiting for async operations to complete...");
    tokio::time::sleep(Duration::from_millis(1000)).await;
   
    // モックの検証
    println!("Verifying mocks...");
    mocks.begin_mock.assert_async().await;
    mocks.create_mock.assert_async().await;
    mocks.commit_mock.assert_async().await;
   
    // 結果の検証
    assert!(result.is_ok(), "Create operation failed: {:?}", result.as_ref().err());

    // 結果の検証
    if let Ok(created) = result {
        // ユーザー名の検証
        assert_eq!(created.username, new_user.username);
        // メールアドレスの検証
        assert_eq!(created.email, new_user.email);
    }
}

// 更新結果の検証
async fn verify_update_result(
    result: &Result<User, UserError>,
    user_id: Uuid,
    update_user: &NewUser,
    mocks: &MockHandles
) {
    // 結果のログ
    match result {
        Ok(user) => println!("Update succeeded: {:?}", user),
        Err(e) => println!("Update failed with error: {:?}", e),
    }

    // モックの検証
    println!("Verifying mocks...");
    mocks.begin_mock.assert_async().await;
    mocks.create_mock.assert_async().await;
    mocks.commit_mock.assert_async().await;

    // 結果の検証
    assert!(result.is_ok(), "Update operation failed: {:?}", result.as_ref().err());

    // 結果の検証
    if let Ok(user) = result {
        // IDの検証
        assert_eq!(user.id, user_id);
        // ユーザー名の検証
        assert_eq!(user.username, update_user.username);
        // メールアドレスの検証
        assert_eq!(user.email, update_user.email);
        // パスワードの検証
        assert_ne!(user.password, update_user.password);
    }
}

// 削除結果の検証
async fn verify_delete_result(result: &Result<(), UserError>, mocks: &MockHandles) {
    // 結果のログ
    match result {
        Ok(_) => println!("Delete succeeded"),
        Err(e) => println!("Delete failed with error: {:?}", e),
    }

    // モックの検証
    println!("Verifying mocks...");
    mocks.begin_mock.assert_async().await;
    mocks.create_mock.assert_async().await;
    mocks.commit_mock.assert_async().await;

    // 結果の検証
    assert!(result.is_ok(), "Delete operation failed: {:?}", result.as_ref().err());
}