use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Generic pagination wrapper returned by all list endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub offset: i64,
    pub limit: i32,
    pub count: i32,
    pub total_count: i64,
    pub data: Vec<T>,
}

/// Site overview — from `GET /v1/sites`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteResponse {
    pub id: Uuid,
    pub name: String,
    /// Used as the Legacy API site name (`/api/s/{internalReference}/`).
    pub internal_reference: String,
}

/// Application info — from `GET /v1/info`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationInfoResponse {
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

/// Error response returned by the Integration API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
