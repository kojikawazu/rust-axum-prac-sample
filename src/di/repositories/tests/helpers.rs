use mockito::Mock;
use uuid::Uuid;
use chrono::Utc;
use serde_json::json;
// モデルのインポート
use crate::models::users::users::{User, NewUser};

// モックのハンドル
pub struct MockHandles {
    // 開始モック
    pub begin_mock: Mock,
    // 作成モック
    pub create_mock: Mock,
    // コミットモック
    pub commit_mock: Mock,
}

// テストユーザーの作成
pub fn create_test_user() -> NewUser {
    // テストユーザーの作成
    NewUser {
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    }
}

// IDつきテストユーザーの作成
pub fn create_test_user_with_id() -> User {
    // テストユーザーの作成
    User {
        id: Uuid::new_v4(),
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        password: "hashed_password".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    }
}

// モックのセットアップ
pub async fn setup_mocks(mock_server: &mut mockito::Server, created_user: &User) -> MockHandles {
    let begin_mock = mock_server
        .mock("POST", "/rest/v1/rpc/begin_transaction")
        .match_header("apikey", "test_key")
        .match_header("Content-Type", "application/json")
        .match_body(mockito::Matcher::Any)
        .with_status(200)
        .with_body(json!({"transaction_id": "test-tx"}).to_string())
        .create_async()
        .await;

    // ユーザー作成のモック
    let create_mock = mock_server
        .mock("POST", "/rest/v1/trans_users")
        .match_header("apikey", "test_key")
        .match_header("Content-Type", "application/json")
        .match_header("Transaction-Id", "test-tx")
        .match_header("Prefer", "return=representation")
        .match_body(mockito::Matcher::Any)
        .with_status(200)
        .with_body(json!([created_user]).to_string())
        .create_async()
        .await;

    // コミットモック
    let commit_mock = mock_server
        .mock("POST", "/rest/v1/rpc/commit_transaction")
        .match_header("apikey", "test_key")
        .match_header("Content-Type", "application/json")
        .match_body(mockito::Matcher::JsonString(
            json!({"transaction_id": "test-tx"}).to_string()
        ))
        .with_status(200)
        .with_body("{}")
        .create_async()
        .await;

    // モックハンドルの返却
    MockHandles {
        begin_mock,
        create_mock,
        commit_mock,
    }
}

// 更新用のモックセットアップ
pub async fn setup_update_mocks(
    mock_server: &mut mockito::Server,
    user_id: &Uuid,
    updated_user: &User
) -> MockHandles {
    // 開始モック
    let begin_mock = mock_server
        .mock("POST", "/rest/v1/rpc/begin_transaction")
        .match_header("apikey", "test_key")
        .match_header("Content-Type", "application/json")
        .match_body(mockito::Matcher::Any)
        .with_status(200)
        .with_body(json!({"transaction_id": "test-tx"}).to_string())
        .create_async()
        .await;

    // ユーザー更新のモック
    let path = format!("/rest/v1/trans_users?id=eq.{}", user_id);
    let update_mock = mock_server
        .mock("PATCH", path.as_str())
        .match_header("apikey", "test_key")
        .match_header("Content-Type", "application/json")
        .match_header("Prefer", "return=representation")
        .match_header("Transaction-Id", "test-tx")
        .match_body(mockito::Matcher::Any)
        .with_status(200)
        .with_body(json!([updated_user]).to_string())
        .create_async()
        .await;

    // コミットモック
    let commit_mock = mock_server
        .mock("POST", "/rest/v1/rpc/commit_transaction")
        .match_header("apikey", "test_key")
        .match_header("Content-Type", "application/json")
        .match_body(mockito::Matcher::JsonString(
            json!({"transaction_id": "test-tx"}).to_string()
        ))
        .with_status(200)
        .with_body("{}")
        .create_async()
        .await;

    // モックハンドルの返却
    MockHandles {
        begin_mock,
        create_mock: update_mock,  // 更新用のモックを割り当て
        commit_mock,
    }
}

// 削除用のモックセットアップ
pub async fn setup_delete_mocks(
    mock_server: &mut mockito::Server,
    user_id: &Uuid
) -> MockHandles {
    // 開始モック
    let begin_mock = mock_server
        .mock("POST", "/rest/v1/rpc/begin_transaction")
        .match_header("apikey", "test_key")
        .match_header("Content-Type", "application/json")
        .match_body(mockito::Matcher::Any)
        .with_status(200)
        .with_body(json!({"transaction_id": "test-tx"}).to_string())
        .create_async()
        .await;

    // ユーザー削除のパス
    let path = format!("/rest/v1/trans_users?id=eq.{}", user_id);
    let delete_mock = mock_server
        .mock("DELETE", path.as_str())
        .match_header("apikey", "test_key")
        .match_header("Transaction-Id", "test-tx")
        .with_status(204)
        .create_async()
        .await;

    // コミットモック
    let commit_mock = mock_server
        .mock("POST", "/rest/v1/rpc/commit_transaction")
        .match_header("apikey", "test_key")
        .match_header("Content-Type", "application/json")
        .match_body(mockito::Matcher::JsonString(
            json!({"transaction_id": "test-tx"}).to_string()
        ))
        .with_status(200)
        .with_body("{}")
        .create_async()
        .await;

    // モックハンドルの返却
    MockHandles {
        begin_mock,
        create_mock: delete_mock,  // 削除用のモックを割り当て
        commit_mock,
    }
}

// 存在しないユーザー検索用のモックセットアップ
pub async fn setup_not_found_mock(
    mock_server: &mut mockito::Server,
    user_id: &Uuid,
) -> Mock {
    mock_server
        .mock("GET", format!("/rest/v1/trans_users?id=eq.{}", user_id).as_str())
        .match_header("apikey", "test_key")
        .match_header("Content-Type", "application/json")
        .with_status(200)
        .with_body(json!([]).to_string())
        .create_async()
        .await
}

// 作成エラー用のモックセットアップ
pub async fn setup_create_error_mock(
    mock_server: &mut mockito::Server,
) -> MockHandles {
    let begin_mock = mock_server
        .mock("POST", "/rest/v1/rpc/begin_transaction")
        .match_header("apikey", "test_key")
        .match_header("Content-Type", "application/json")
        .match_body(mockito::Matcher::Any)
        .with_status(200)
        .with_body(json!({"transaction_id": "test-tx"}).to_string())
        .create_async()
        .await;

    let create_mock = mock_server
        .mock("POST", "/rest/v1/trans_users")
        .match_header("apikey", "test_key")
        .match_header("Content-Type", "application/json")
        .match_header("Transaction-Id", "test-tx")
        .match_body(mockito::Matcher::Any)
        .with_status(400)
        .with_body(json!({
            "message": "Invalid user data"
        }).to_string())
        .create_async()
        .await;

    let commit_mock = mock_server
        .mock("POST", "/rest/v1/rpc/commit_transaction")
        .match_header("apikey", "test_key")
        .match_header("Content-Type", "application/json")
        .match_body(mockito::Matcher::JsonString(
            json!({"transaction_id": "test-tx"}).to_string()
        ))
        .with_status(200)
        .with_body("{}")
        .create_async()
        .await;

    MockHandles {
        begin_mock,
        create_mock,
        commit_mock,
    }
}