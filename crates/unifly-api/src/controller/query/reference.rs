use super::super::*;

impl Controller {
    pub async fn list_vpn_servers(&self) -> Result<Vec<VpnServer>, CoreError> {
        let (client, site_id) = integration_site_context(self, "list_vpn_servers").await?;
        let raw = client
            .paginate_all(200, |offset, limit| {
                client.list_vpn_servers(&site_id, offset, limit)
            })
            .await?;
        Ok(raw
            .into_iter()
            .map(|server| VpnServer {
                id: parse_integration_entity_id(&server.fields),
                name: server
                    .fields
                    .get("name")
                    .and_then(|value| value.as_str())
                    .map(String::from),
                server_type: server
                    .fields
                    .get("type")
                    .or_else(|| server.fields.get("serverType"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("UNKNOWN")
                    .to_owned(),
                enabled: server
                    .fields
                    .get("enabled")
                    .and_then(serde_json::Value::as_bool),
            })
            .collect())
    }

    pub async fn list_vpn_tunnels(&self) -> Result<Vec<VpnTunnel>, CoreError> {
        let (client, site_id) = integration_site_context(self, "list_vpn_tunnels").await?;
        let raw = client
            .paginate_all(200, |offset, limit| {
                client.list_vpn_tunnels(&site_id, offset, limit)
            })
            .await?;
        Ok(raw
            .into_iter()
            .map(|tunnel| VpnTunnel {
                id: parse_integration_entity_id(&tunnel.fields),
                name: tunnel
                    .fields
                    .get("name")
                    .and_then(|value| value.as_str())
                    .map(String::from),
                tunnel_type: tunnel
                    .fields
                    .get("type")
                    .or_else(|| tunnel.fields.get("tunnelType"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("UNKNOWN")
                    .to_owned(),
                enabled: tunnel
                    .fields
                    .get("enabled")
                    .and_then(serde_json::Value::as_bool),
            })
            .collect())
    }

    pub async fn list_wans(&self) -> Result<Vec<WanInterface>, CoreError> {
        let (client, site_id) = integration_site_context(self, "list_wans").await?;
        let raw = client
            .paginate_all(200, |offset, limit| {
                client.list_wans(&site_id, offset, limit)
            })
            .await?;
        Ok(raw
            .into_iter()
            .map(|wan| {
                let parse_ip = |key: &str| -> Option<std::net::IpAddr> {
                    wan.fields
                        .get(key)
                        .and_then(|value| value.as_str())
                        .and_then(|value| value.parse().ok())
                };
                let dns = wan
                    .fields
                    .get("dns")
                    .and_then(|value| value.as_array())
                    .map(|values| {
                        values
                            .iter()
                            .filter_map(|value| value.as_str().and_then(|value| value.parse().ok()))
                            .collect()
                    })
                    .unwrap_or_default();
                WanInterface {
                    id: parse_integration_entity_id(&wan.fields),
                    name: wan
                        .fields
                        .get("name")
                        .and_then(|value| value.as_str())
                        .map(String::from),
                    ip: parse_ip("ipAddress").or_else(|| parse_ip("ip")),
                    gateway: parse_ip("gateway"),
                    dns,
                }
            })
            .collect())
    }

    pub async fn list_dpi_categories(&self) -> Result<Vec<DpiCategory>, CoreError> {
        let (client, site_id) = integration_site_context(self, "list_dpi_categories").await?;
        let raw = client
            .paginate_all(200, |offset, limit| {
                client.list_dpi_categories(&site_id, offset, limit)
            })
            .await?;
        Ok(raw
            .into_iter()
            .map(|category| {
                #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
                let id = category
                    .fields
                    .get("id")
                    .and_then(serde_json::Value::as_u64)
                    .unwrap_or(0) as u32;
                DpiCategory {
                    id,
                    name: category
                        .fields
                        .get("name")
                        .and_then(|value| value.as_str())
                        .unwrap_or("Unknown")
                        .to_owned(),
                    tx_bytes: category
                        .fields
                        .get("txBytes")
                        .and_then(serde_json::Value::as_u64)
                        .unwrap_or(0),
                    rx_bytes: category
                        .fields
                        .get("rxBytes")
                        .and_then(serde_json::Value::as_u64)
                        .unwrap_or(0),
                    apps: Vec::new(),
                }
            })
            .collect())
    }

    pub async fn list_dpi_applications(&self) -> Result<Vec<DpiApplication>, CoreError> {
        let (client, site_id) = integration_site_context(self, "list_dpi_applications").await?;
        let raw = client
            .paginate_all(200, |offset, limit| {
                client.list_dpi_applications(&site_id, offset, limit)
            })
            .await?;
        Ok(raw
            .into_iter()
            .map(|application| {
                #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
                let id = application
                    .fields
                    .get("id")
                    .and_then(serde_json::Value::as_u64)
                    .unwrap_or(0) as u32;
                DpiApplication {
                    id,
                    name: application
                        .fields
                        .get("name")
                        .and_then(|value| value.as_str())
                        .unwrap_or("Unknown")
                        .to_owned(),
                    #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
                    category_id: application
                        .fields
                        .get("categoryId")
                        .and_then(serde_json::Value::as_u64)
                        .unwrap_or(0) as u32,
                    tx_bytes: application
                        .fields
                        .get("txBytes")
                        .and_then(serde_json::Value::as_u64)
                        .unwrap_or(0),
                    rx_bytes: application
                        .fields
                        .get("rxBytes")
                        .and_then(serde_json::Value::as_u64)
                        .unwrap_or(0),
                }
            })
            .collect())
    }

    pub async fn list_radius_profiles(&self) -> Result<Vec<RadiusProfile>, CoreError> {
        let (client, site_id) = integration_site_context(self, "list_radius_profiles").await?;
        let raw = client
            .paginate_all(200, |offset, limit| {
                client.list_radius_profiles(&site_id, offset, limit)
            })
            .await?;
        Ok(raw
            .into_iter()
            .map(|profile| RadiusProfile {
                id: parse_integration_entity_id(&profile.fields),
                name: profile
                    .fields
                    .get("name")
                    .and_then(|value| value.as_str())
                    .unwrap_or("Unknown")
                    .to_owned(),
            })
            .collect())
    }

    pub async fn list_countries(&self) -> Result<Vec<Country>, CoreError> {
        let client = integration_client_context(self, "list_countries").await?;
        let raw = client
            .paginate_all(200, |offset, limit| client.list_countries(offset, limit))
            .await?;
        Ok(raw
            .into_iter()
            .map(|country| Country {
                code: country
                    .fields
                    .get("code")
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .to_owned(),
                name: country
                    .fields
                    .get("name")
                    .and_then(|value| value.as_str())
                    .unwrap_or("Unknown")
                    .to_owned(),
            })
            .collect())
    }

    pub async fn get_network_references(
        &self,
        network_id: &EntityId,
    ) -> Result<serde_json::Value, CoreError> {
        let (client, site_id) = integration_site_context(self, "get_network_references").await?;
        let uuid = require_uuid(network_id)?;
        let refs = client.get_network_references(&site_id, &uuid).await?;
        Ok(serde_json::to_value(refs).unwrap_or_default())
    }
}

fn parse_integration_entity_id(
    fields: &std::collections::HashMap<String, serde_json::Value>,
) -> EntityId {
    fields
        .get("id")
        .and_then(|value| value.as_str())
        .and_then(|value| uuid::Uuid::parse_str(value).ok())
        .map_or_else(|| EntityId::Legacy("unknown".into()), EntityId::Uuid)
}

#[cfg(test)]
mod tests {
    use super::parse_integration_entity_id;
    use crate::EntityId;

    #[test]
    fn parse_integration_entity_id_prefers_valid_uuid() {
        let fields: std::collections::HashMap<String, serde_json::Value> =
            serde_json::from_value(serde_json::json!({
                "id": "11111111-1111-1111-1111-111111111111"
            }))
            .expect("hash map");

        let id = parse_integration_entity_id(&fields);
        assert_eq!(id.to_string(), "11111111-1111-1111-1111-111111111111");
        assert!(id.as_uuid().is_some());
    }

    #[test]
    fn parse_integration_entity_id_falls_back_to_legacy_unknown() {
        let fields: std::collections::HashMap<String, serde_json::Value> =
            serde_json::from_value(serde_json::json!({})).expect("hash map");
        let id = parse_integration_entity_id(&fields);

        assert_eq!(id, EntityId::Legacy("unknown".into()));
    }
}
