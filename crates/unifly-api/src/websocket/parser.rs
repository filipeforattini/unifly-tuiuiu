use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

/// A parsed event from the UniFi WebSocket stream.
///
/// Uses `#[serde(flatten)]` to capture all fields beyond the core set,
/// so nothing from the controller is silently dropped.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiEvent {
    /// Event key, e.g. `"EVT_WU_Connected"`, `"EVT_SW_Disconnected"`.
    pub key: String,

    /// Subsystem that emitted the event: `"wlan"`, `"lan"`, `"sta"`, `"gw"`, etc.
    pub subsystem: String,

    /// Site ID this event belongs to.
    pub site_id: String,

    /// Human-readable event message, if present.
    /// The controller sends `"msg"` in most payloads; `"message"` is a rarer variant.
    #[serde(default, alias = "msg")]
    pub message: Option<String>,

    /// ISO-8601 timestamp from the controller.
    #[serde(default)]
    pub datetime: Option<String>,

    /// All remaining fields the controller sends.
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct WsEnvelope {
    #[allow(dead_code)]
    meta: WsMeta,
    data: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct WsMeta {
    #[allow(dead_code)]
    rc: String,
    #[serde(default)]
    message: Option<String>,
}

pub(in crate::websocket) fn parse_and_broadcast(
    text: &str,
    event_tx: &broadcast::Sender<Arc<UnifiEvent>>,
) {
    let envelope: WsEnvelope = match serde_json::from_str(text) {
        Ok(envelope) => envelope,
        Err(error) => {
            tracing::debug!(error = %error, "Failed to parse WebSocket envelope");
            return;
        }
    };

    let msg_type = envelope.meta.message.as_deref().unwrap_or("");

    for data in envelope.data {
        let event = match msg_type {
            "events" => match serde_json::from_value::<UnifiEvent>(data.clone()) {
                Ok(event) => event,
                Err(error) => {
                    tracing::debug!(
                        error = %error,
                        msg_type,
                        "Could not deserialize event, constructing from raw data"
                    );
                    event_from_raw(msg_type, &data)
                }
            },
            _ => event_from_raw(msg_type, &data),
        };

        let _ = event_tx.send(Arc::new(event));
    }
}

fn event_from_raw(msg_type: &str, data: &serde_json::Value) -> UnifiEvent {
    UnifiEvent {
        key: data["key"].as_str().unwrap_or(msg_type).to_string(),
        subsystem: data["subsystem"].as_str().unwrap_or("unknown").to_string(),
        site_id: data["site_id"].as_str().unwrap_or("").to_string(),
        message: data["msg"]
            .as_str()
            .or_else(|| data["message"].as_str())
            .map(String::from),
        datetime: data["datetime"].as_str().map(String::from),
        extra: data.clone(),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn parse_event_from_raw_json() {
        let data = serde_json::json!({
            "key": "EVT_WU_Connected",
            "subsystem": "wlan",
            "site_id": "abc123",
            "msg": "User[aa:bb:cc:dd:ee:ff] connected",
            "datetime": "2026-02-10T12:00:00Z",
            "user": "aa:bb:cc:dd:ee:ff",
            "ssid": "MyNetwork"
        });

        let event = event_from_raw("events", &data);
        assert_eq!(event.key, "EVT_WU_Connected");
        assert_eq!(event.subsystem, "wlan");
        assert_eq!(event.site_id, "abc123");
        assert_eq!(
            event.message.as_deref(),
            Some("User[aa:bb:cc:dd:ee:ff] connected")
        );
        assert_eq!(event.datetime.as_deref(), Some("2026-02-10T12:00:00Z"));
    }

    #[test]
    fn parse_sync_event_from_raw_json() {
        let data = serde_json::json!({
            "mac": "aa:bb:cc:dd:ee:ff",
            "state": 1,
            "site_id": "site1"
        });

        let event = event_from_raw("device:sync", &data);
        assert_eq!(event.key, "device:sync");
        assert_eq!(event.subsystem, "unknown");
        assert_eq!(event.site_id, "site1");
    }

    #[test]
    fn deserialize_unifi_event() {
        let json = r#"{
            "key": "EVT_SW_Disconnected",
            "subsystem": "lan",
            "site_id": "default",
            "message": "Switch lost contact",
            "datetime": "2026-02-10T13:00:00Z",
            "sw": "aa:bb:cc:dd:ee:ff",
            "port": 4
        }"#;

        let event: UnifiEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.key, "EVT_SW_Disconnected");
        assert_eq!(event.subsystem, "lan");
        assert_eq!(event.site_id, "default");
        assert_eq!(event.message.as_deref(), Some("Switch lost contact"));
        assert_eq!(event.extra["sw"], "aa:bb:cc:dd:ee:ff");
        assert_eq!(event.extra["port"], 4);
    }

    #[test]
    fn deserialize_unifi_event_msg_alias() {
        let json = r#"{
            "key": "EVT_WU_Connected",
            "subsystem": "wlan",
            "site_id": "abc123",
            "msg": "User[aa:bb:cc:dd:ee:ff] connected",
            "datetime": "2026-02-10T12:00:00Z"
        }"#;

        let event: UnifiEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event.message.as_deref(),
            Some("User[aa:bb:cc:dd:ee:ff] connected")
        );
    }

    #[test]
    fn parse_and_broadcast_events_message() {
        let (tx, mut rx) = broadcast::channel(16);

        let raw = serde_json::json!({
            "meta": { "rc": "ok", "message": "events" },
            "data": [{
                "key": "EVT_WU_Connected",
                "subsystem": "wlan",
                "site_id": "default",
                "msg": "Client connected",
                "user": "aa:bb:cc:dd:ee:ff"
            }]
        });

        parse_and_broadcast(&raw.to_string(), &tx);

        let event = rx.try_recv().unwrap();
        assert_eq!(event.key, "EVT_WU_Connected");
        assert_eq!(event.subsystem, "wlan");
    }

    #[test]
    fn parse_and_broadcast_sync_message() {
        let (tx, mut rx) = broadcast::channel(16);

        let raw = serde_json::json!({
            "meta": { "rc": "ok", "message": "device:sync" },
            "data": [{
                "mac": "aa:bb:cc:dd:ee:ff",
                "state": 1,
                "site_id": "site1"
            }]
        });

        parse_and_broadcast(&raw.to_string(), &tx);

        let event = rx.try_recv().unwrap();
        assert_eq!(event.key, "device:sync");
        assert_eq!(event.site_id, "site1");
    }

    #[test]
    fn parse_and_broadcast_malformed_json() {
        let (tx, mut rx) = broadcast::channel::<Arc<UnifiEvent>>(16);

        parse_and_broadcast("not json at all", &tx);

        assert!(rx.try_recv().is_err());
    }
}
