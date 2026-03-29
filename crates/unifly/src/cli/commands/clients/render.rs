use std::sync::Arc;

use tabled::Tabled;
use unifly_api::Client;

use crate::cli::output::Painter;

#[derive(Tabled)]
pub(super) struct ClientRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "IP")]
    ip: String,
    #[tabled(rename = "Type")]
    ctype: String,
    #[tabled(rename = "Uplink")]
    uplink: String,
}

pub(super) fn client_row(client: &Arc<Client>, painter: &Painter) -> ClientRow {
    let name = client
        .name
        .clone()
        .or_else(|| client.hostname.clone())
        .unwrap_or_default();
    ClientRow {
        name: painter.name(&name),
        ip: painter.ip(&client.ip.map(|ip| ip.to_string()).unwrap_or_default()),
        ctype: painter.muted(&format!("{:?}", client.client_type)),
        uplink: painter.mac(
            &client
                .uplink_device_mac
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default(),
        ),
    }
}

pub(super) fn detail(client: &Arc<Client>) -> String {
    let mut lines = vec![
        format!("ID:        {}", client.id),
        format!("Name:      {}", client.name.as_deref().unwrap_or("-")),
        format!("Hostname:  {}", client.hostname.as_deref().unwrap_or("-")),
        format!("MAC:       {}", client.mac),
        format!(
            "IP:        {}",
            client.ip.map_or_else(|| "-".into(), |ip| ip.to_string())
        ),
        format!("Type:      {:?}", client.client_type),
        format!("Guest:     {}", client.is_guest),
        format!("Blocked:   {}", client.blocked),
    ];
    if client.use_fixedip {
        lines.push(format!(
            "Fixed IP:  {}",
            client.fixed_ip.map_or("-".into(), |ip| ip.to_string())
        ));
    }
    if let Some(wireless) = &client.wireless {
        lines.push(format!(
            "SSID:      {}",
            wireless.ssid.as_deref().unwrap_or("-")
        ));
        if let Some(signal) = wireless.signal_dbm {
            lines.push(format!("Signal:    {signal} dBm"));
        }
    }
    if let Some(os_name) = &client.os_name {
        lines.push(format!("OS:        {os_name}"));
    }
    lines.join("\n")
}
