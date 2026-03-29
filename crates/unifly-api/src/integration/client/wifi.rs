use uuid::Uuid;

use super::{Error, IntegrationClient, types};

impl IntegrationClient {
    // ── WiFi Broadcasts ──────────────────────────────────────────────

    pub async fn list_wifi_broadcasts(
        &self,
        site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::WifiBroadcastResponse>, Error> {
        self.get_with_params(
            &format!("v1/sites/{site_id}/wifi/broadcasts"),
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    pub async fn get_wifi_broadcast(
        &self,
        site_id: &Uuid,
        broadcast_id: &Uuid,
    ) -> Result<types::WifiBroadcastDetailsResponse, Error> {
        self.get(&format!(
            "v1/sites/{site_id}/wifi/broadcasts/{broadcast_id}"
        ))
        .await
    }

    pub async fn create_wifi_broadcast(
        &self,
        site_id: &Uuid,
        body: &types::WifiBroadcastCreateUpdate,
    ) -> Result<types::WifiBroadcastDetailsResponse, Error> {
        self.post(&format!("v1/sites/{site_id}/wifi/broadcasts"), body)
            .await
    }

    pub async fn update_wifi_broadcast(
        &self,
        site_id: &Uuid,
        broadcast_id: &Uuid,
        body: &types::WifiBroadcastCreateUpdate,
    ) -> Result<types::WifiBroadcastDetailsResponse, Error> {
        self.put(
            &format!("v1/sites/{site_id}/wifi/broadcasts/{broadcast_id}"),
            body,
        )
        .await
    }

    pub async fn delete_wifi_broadcast(
        &self,
        site_id: &Uuid,
        broadcast_id: &Uuid,
    ) -> Result<(), Error> {
        self.delete(&format!(
            "v1/sites/{site_id}/wifi/broadcasts/{broadcast_id}"
        ))
        .await
    }
}
