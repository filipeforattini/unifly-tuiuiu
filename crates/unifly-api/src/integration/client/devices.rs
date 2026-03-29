use serde::Serialize;
use uuid::Uuid;

use super::{Error, IntegrationClient, types};

impl IntegrationClient {
    // ── Devices ──────────────────────────────────────────────────────

    pub async fn list_devices(
        &self,
        site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::DeviceResponse>, Error> {
        self.get_with_params(
            &format!("v1/sites/{site_id}/devices"),
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    pub async fn get_device(
        &self,
        site_id: &Uuid,
        device_id: &Uuid,
    ) -> Result<types::DeviceDetailsResponse, Error> {
        self.get(&format!("v1/sites/{site_id}/devices/{device_id}"))
            .await
    }

    pub async fn get_device_statistics(
        &self,
        site_id: &Uuid,
        device_id: &Uuid,
    ) -> Result<types::DeviceStatisticsResponse, Error> {
        self.get(&format!(
            "v1/sites/{site_id}/devices/{device_id}/statistics/latest"
        ))
        .await
    }

    pub async fn adopt_device(
        &self,
        site_id: &Uuid,
        mac: &str,
        ignore_device_limit: bool,
    ) -> Result<types::DeviceDetailsResponse, Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Body<'a> {
            mac_address: &'a str,
            ignore_device_limit: bool,
        }

        self.post(
            &format!("v1/sites/{site_id}/devices"),
            &Body {
                mac_address: mac,
                ignore_device_limit,
            },
        )
        .await
    }

    pub async fn remove_device(&self, site_id: &Uuid, device_id: &Uuid) -> Result<(), Error> {
        self.delete(&format!("v1/sites/{site_id}/devices/{device_id}"))
            .await
    }

    pub async fn device_action(
        &self,
        site_id: &Uuid,
        device_id: &Uuid,
        action: &str,
    ) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Body<'a> {
            action: &'a str,
        }

        self.post_no_response(
            &format!("v1/sites/{site_id}/devices/{device_id}/actions"),
            &Body { action },
        )
        .await
    }

    pub async fn port_action(
        &self,
        site_id: &Uuid,
        device_id: &Uuid,
        port_idx: u32,
        action: &str,
    ) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Body<'a> {
            action: &'a str,
        }

        self.post_no_response(
            &format!("v1/sites/{site_id}/devices/{device_id}/interfaces/ports/{port_idx}/actions"),
            &Body { action },
        )
        .await
    }

    pub async fn list_pending_devices(
        &self,
        _site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::PendingDeviceResponse>, Error> {
        self.get_with_params(
            "v1/pending-devices",
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    pub async fn list_device_tags(
        &self,
        site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::DeviceTagResponse>, Error> {
        self.get_with_params(
            &format!("v1/sites/{site_id}/device-tags"),
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }
}
