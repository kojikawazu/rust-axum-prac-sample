use mockito::Mock;
use reqwest::Client;
use uuid::Uuid;
// リポジトリのインポート
use crate::di::repositories::user_repository::{UserRepository, UserRepositoryTrait};
// エラーのインポート
use crate::errors::users::user_error::UserError;
// モデルのインポート
use crate::models::users::users::{User};
// ヘルパーのインポート
use super::helpers::{
    create_test_user,   
    setup_not_found_mock,
    setup_create_error_mock,
    MockHandles
};

// ユーザーIDが見つからない場合のテスト
#[tokio::test]
async fn test_find_by_id_not_found() {
    // モックサーバーの作成
    let mut mock_server = mockito::Server::new_async().await;
    // モックURLの取得
    let mock_url = mock_server.url();
    // ユーザーIDの生成
    let user_id = Uuid::new_v4();
    println!("Setting up not found mock for user_id: {}", user_id);
    // モックのセットアップ
    let mock = setup_not_found_mock(&mut mock_server, &user_id).await;

    // クライアントの作成
    let client = Client::new();
    // リポジトリの作成
    let repository = UserRepository::new(
        client,
        mock_url,
        "test_key".to_string(),
    );
    // ユーザーIDを使用してユーザーを検索
    println!("Executing find_by_id operation...");
    let result = repository.find_by_id(user_id).await;
    // 結果の検証
    verify_not_found_result(&result, &mock).await;
}

// ユーザー作成エラーのテスト
#[tokio::test]
async fn test_create_error() {
    // モックサーバーの作成
    let mut mock_server = mockito::Server::new_async().await;
    // モックURLの取得
    let mock_url = mock_server.url();
    // テストユーザーの作成
    let new_user = create_test_user();
    // モックのセットアップ
    println!("Setting up create error mocks");
    let mocks = setup_create_error_mock(&mut mock_server).await;

    // クライアントの作成
    let client = Client::new();
    // リポジトリの作成
    let repository = UserRepository::new(
        client,
        mock_url,
        "test_key".to_string(),
    );
    // 無効なデータを使用してユーザーを作成
    println!("Executing create operation with invalid data...");
    let result = repository.create(new_user).await;
    // 結果の検証
    verify_create_error_result(&result, &mocks).await;
}

// 検証用のヘルパー関数
async fn verify_not_found_result(
    result: &Result<User, UserError>,
    mock: &Mock,
) {
    // 結果の検証
    match result {
        Ok(_) => println!("Unexpectedly found user"),
        Err(e) => println!("Expected error occurred: {:?}", e),
    }

    // モックの検証
    println!("Verifying mock...");
    mock.assert_async().await;

    assert!(result.is_err(), "Expected not found error");
    if let Err(e) = result {
        // エラーの種類を確認する処理を追加可能
        println!("Received error: {:?}", e);
    }
}

// ユーザー作成エラーの検証
async fn verify_create_error_result(
    result: &Result<User, UserError>,
    mocks: &MockHandles,
) {
    // 結果の検証
    match result {
        Ok(_) => println!("Unexpectedly created user"),
        Err(e) => println!("Expected error occurred: {:?}", e),
    }

    // モックの検証
    println!("Verifying mocks...");
    mocks.begin_mock.assert_async().await;
    mocks.create_mock.assert_async().await;

    assert!(result.is_err(), "Expected creation error");
    if let Err(e) = result {
        // エラーの種類を確認する処理を追加可能
        println!("Received error: {:?}", e);
    }
}