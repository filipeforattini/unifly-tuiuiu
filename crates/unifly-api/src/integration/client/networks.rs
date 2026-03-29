use uuid::Uuid;

use super::{Error, IntegrationClient, types};

impl IntegrationClient {
    // ── Networks ─────────────────────────────────────────────────────

    pub async fn list_networks(
        &self,
        site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::NetworkResponse>, Error> {
        self.get_with_params(
            &format!("v1/sites/{site_id}/networks"),
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    pub async fn get_network(
        &self,
        site_id: &Uuid,
        network_id: &Uuid,
    ) -> Result<types::NetworkDetailsResponse, Error> {
        self.get(&format!("v1/sites/{site_id}/networks/{network_id}"))
            .await
    }

    pub async fn create_network(
        &self,
        site_id: &Uuid,
        body: &types::NetworkCreateUpdate,
    ) -> Result<types::NetworkDetailsResponse, Error> {
        self.post(&format!("v1/sites/{site_id}/networks"), body)
            .await
    }

    pub async fn update_network(
        &self,
        site_id: &Uuid,
        network_id: &Uuid,
        body: &types::NetworkCreateUpdate,
    ) -> Result<types::NetworkDetailsResponse, Error> {
        self.put(&format!("v1/sites/{site_id}/networks/{network_id}"), body)
            .await
    }

    pub async fn delete_network(&self, site_id: &Uuid, network_id: &Uuid) -> Result<(), Error> {
        self.delete(&format!("v1/sites/{site_id}/networks/{network_id}"))
            .await
    }

    pub async fn get_network_references(
        &self,
        site_id: &Uuid,
        network_id: &Uuid,
    ) -> Result<types::NetworkReferencesResponse, Error> {
        self.get(&format!(
            "v1/sites/{site_id}/networks/{network_id}/references"
        ))
        .await
    }
}
