use super::{Error, IntegrationClient, types};

impl IntegrationClient {
    // ── System Info ──────────────────────────────────────────────────

    pub async fn get_info(&self) -> Result<types::ApplicationInfoResponse, Error> {
        self.get("v1/info").await
    }

    // ── Sites ────────────────────────────────────────────────────────

    pub async fn list_sites(
        &self,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::SiteResponse>, Error> {
        self.get_with_params(
            "v1/sites",
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }
}
