// 共通モデルのエントリーポイント
use chrono::NaiveDateTime;
use uuid::Uuid;
use utoipa::ToSchema;

// NaiveDateTimeのToSchema実装
#[derive(ToSchema)]
#[schema(value_type = String, format = "date-time", example = "2024-01-01T00:00:00")]
#[allow(dead_code)]
pub struct NaiveDateTimeWrapper(pub NaiveDateTime);

// UuidのToSchema実装
#[derive(ToSchema)]
#[schema(value_type = String, format = "uuid", example = "123e4567-e89b-12d3-a456-426614174000")]
#[allow(dead_code)]
pub struct UuidWrapper(pub Uuid);