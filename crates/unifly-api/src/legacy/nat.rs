// NAT policy operations via v2 API
//
// The Integration API does not expose NAT write endpoints on many
// controller versions. The v2 API (`/v2/api/site/{site}/nat`) provides
// full CRUD and is used here instead.

use serde_json::Value;
use tracing::debug;

use super::LegacyClient;
use crate::error::Error;

impl LegacyClient {
    /// List all NAT rules via the v2 API.
    ///
    /// `GET /v2/api/site/{site}/nat`
    pub async fn list_nat_rules(&self) -> Result<Vec<Value>, Error> {
        let url = self.site_url_v2("nat");
        debug!("listing NAT rules (v2)");
        let val = self.get_raw(url).await?;
        Ok(val.as_array().cloned().unwrap_or_default())
    }

    /// Create a NAT rule via the v2 API.
    ///
    /// `POST /v2/api/site/{site}/nat`
    pub async fn create_nat_rule(&self, body: &Value) -> Result<Value, Error> {
        let path = format!("v2/api/site/{}/nat", self.site());
        debug!("creating NAT rule (v2)");
        self.raw_post(&path, body).await
    }

    /// Update a NAT rule via the v2 API.
    ///
    /// `PUT /v2/api/site/{site}/nat/{rule_id}`
    pub async fn update_nat_rule(&self, rule_id: &str, body: &Value) -> Result<Value, Error> {
        let path = format!("v2/api/site/{}/nat/{rule_id}", self.site());
        debug!(rule_id, "updating NAT rule (v2)");
        self.raw_put(&path, body).await
    }

    /// Delete a NAT rule via the v2 API.
    ///
    /// `DELETE /v2/api/site/{site}/nat/{rule_id}`
    pub async fn delete_nat_rule(&self, rule_id: &str) -> Result<(), Error> {
        let path = format!("v2/api/site/{}/nat/{rule_id}", self.site());
        debug!(rule_id, "deleting NAT rule (v2)");
        self.raw_delete(&path).await
    }
}
