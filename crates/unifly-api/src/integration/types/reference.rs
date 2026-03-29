use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// DPI category — from `GET /v1/sites/{siteId}/dpi/categories`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DpiCategoryResponse {
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

/// DPI application — from `GET /v1/sites/{siteId}/dpi/applications`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DpiApplicationResponse {
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

/// VPN server — from `GET /v1/sites/{siteId}/vpn/servers`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpnServerResponse {
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

/// VPN tunnel — from `GET /v1/sites/{siteId}/vpn/tunnels`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpnTunnelResponse {
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

/// WAN configuration — from `GET /v1/sites/{siteId}/wan`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WanResponse {
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

/// RADIUS profile — from `GET /v1/sites/{siteId}/radius/profiles`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadiusProfileResponse {
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

/// Country metadata — from `GET /v1/countries`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryResponse {
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}
