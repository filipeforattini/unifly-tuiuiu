// Session API system log endpoints (v2)
//
// Covers client connection timeline queries from the raw JSON system-log
// surface.

use serde_json::Value;
use tracing::debug;

use crate::error::Error;
use crate::session::client::SessionClient;

impl SessionClient {
    /// Get a client's connection timeline: connects, disconnects, and roams.
    ///
    /// `GET /v2/api/site/{site}/system-log/client-connection/{mac}`
    ///
    /// **Quirk:** the MAC must appear in both the URL path and the
    /// `?mac=` query parameter.
    pub async fn get_client_roams(
        &self,
        mac: &str,
        limit: Option<u32>,
    ) -> Result<Vec<Value>, Error> {
        let limit = limit.unwrap_or(200);
        let path = format!(
            "system-log/client-connection/{mac}?mac={mac}&separateConnectionSignalParam=false&limit={limit}"
        );
        let url = self.site_url_v2(&path);
        debug!(mac, limit, "fetching client roam timeline");
        let value = self.get_raw(url).await?;
        Ok(match value {
            Value::Array(items) => items,
            other => vec![other],
        })
    }
}
