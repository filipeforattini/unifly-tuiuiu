use serde::Serialize;
use uuid::Uuid;

use super::{Error, IntegrationClient, types};

impl IntegrationClient {
    // ── Clients ──────────────────────────────────────────────────────

    pub async fn list_clients(
        &self,
        site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::ClientResponse>, Error> {
        self.get_with_params(
            &format!("v1/sites/{site_id}/clients"),
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    pub async fn get_client(
        &self,
        site_id: &Uuid,
        client_id: &Uuid,
    ) -> Result<types::ClientDetailsResponse, Error> {
        self.get(&format!("v1/sites/{site_id}/clients/{client_id}"))
            .await
    }

    pub async fn client_action(
        &self,
        site_id: &Uuid,
        client_id: &Uuid,
        action: &str,
    ) -> Result<types::ClientActionResponse, Error> {
        #[derive(Serialize)]
        struct Body<'a> {
            action: &'a str,
        }

        self.post(
            &format!("v1/sites/{site_id}/clients/{client_id}/actions"),
            &Body { action },
        )
        .await
    }
}
