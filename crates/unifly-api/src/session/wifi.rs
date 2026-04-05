// Session API Wi-Fi observability endpoints
//
// Covers neighboring AP scans, regulatory channel availability, and
// per-client Wi-Fi experience metrics from the v2 wifiman surface.

use serde_json::Value;
use tracing::debug;

use crate::error::Error;
use crate::session::client::SessionClient;
use crate::session::models::{ChannelAvailability, RogueAp};

impl SessionClient {
    /// List neighboring / rogue access points detected by your APs.
    ///
    /// `GET /api/s/{site}/stat/rogueap`
    ///
    /// **Quirk:** `within_secs` uses Unix epoch seconds semantics, not
    /// milliseconds like many other UniFi stats routes.
    pub async fn list_rogue_aps(&self, within_secs: Option<i64>) -> Result<Vec<RogueAp>, Error> {
        let path = match within_secs {
            Some(secs) => format!("stat/rogueap?within={secs}"),
            None => "stat/rogueap".to_string(),
        };
        let url = self.site_url(&path);
        debug!(?within_secs, "listing rogue and neighboring APs");
        self.get(url).await
    }

    /// List per-radio regulatory channel availability.
    ///
    /// `GET /api/s/{site}/stat/current-channel`
    pub async fn list_channels(&self) -> Result<Vec<ChannelAvailability>, Error> {
        let url = self.site_url("stat/current-channel");
        debug!("listing regulatory channel availability");
        self.get(url).await
    }

    /// Get per-client live Wi-Fi experience metrics.
    ///
    /// `GET /v2/api/site/{site}/wifiman/{client_ip}/`
    ///
    /// **Quirk:** Band codes from this endpoint (`2.4g`, `5g`, `6g`)
    /// differ from `stat/sta` (`ng`, `na`, `6e`).
    pub async fn get_client_wifi_experience(&self, client_ip: &str) -> Result<Value, Error> {
        let path = format!("wifiman/{client_ip}/");
        let url = self.site_url_v2(&path);
        debug!(client_ip, "fetching client Wi-Fi experience");
        self.get_raw(url).await
    }
}
