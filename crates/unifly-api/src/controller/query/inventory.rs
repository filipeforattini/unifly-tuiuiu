use super::super::*;

impl Controller {
    pub async fn list_pending_devices(&self) -> Result<Vec<serde_json::Value>, CoreError> {
        let integration = self.inner.integration_client.lock().await.clone();
        let site_id = *self.inner.site_id.lock().await;

        if let (Some(client), Some(site_id)) = (integration, site_id) {
            let raw = client
                .paginate_all(200, |offset, limit| {
                    client.list_pending_devices(&site_id, offset, limit)
                })
                .await?;
            return Ok(raw
                .into_iter()
                .map(|value| serde_json::to_value(value).unwrap_or_default())
                .collect());
        }

        let snapshot = self.devices_snapshot();
        Ok(snapshot
            .iter()
            .filter(|device| device.state == crate::model::DeviceState::PendingAdoption)
            .map(|device| serde_json::to_value(device.as_ref()).unwrap_or_default())
            .collect())
    }

    pub async fn list_device_tags(&self) -> Result<Vec<serde_json::Value>, CoreError> {
        let integration = self.inner.integration_client.lock().await.clone();
        let site_id = *self.inner.site_id.lock().await;

        if let (Some(client), Some(site_id)) = (integration, site_id) {
            let raw = client
                .paginate_all(200, |offset, limit| {
                    client.list_device_tags(&site_id, offset, limit)
                })
                .await?;
            return Ok(raw
                .into_iter()
                .map(|value| serde_json::to_value(value).unwrap_or_default())
                .collect());
        }

        Ok(Vec::new())
    }
}
