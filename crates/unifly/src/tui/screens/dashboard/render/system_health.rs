use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use unifly_api::ClientType;

use crate::tui::theme;
use crate::tui::widgets::bytes_fmt;

use super::super::DashboardScreen;
use super::super::helpers::{fmt_rate_compact, truncate_text};

impl DashboardScreen {
    /// Connectivity card - subsystem state and per-subsystem activity.
    #[allow(clippy::too_many_lines)]
    pub(super) fn render_system_health(
        &self,
        frame: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
    ) {
        let block = Block::default()
            .title(Span::styled(" Connectivity ", theme::title_style()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_default());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let w = usize::from(inner.width);
        let content_w = w.saturating_sub(8);
        let bar_width = u16::try_from(w.saturating_sub(13).clamp(6, 14)).unwrap_or(8);
        let mut lines = Vec::new();

        let status_color = |sub: &str| -> ratatui::style::Color {
            match self
                .health
                .iter()
                .find(|h| h.subsystem == sub)
                .map(|h| h.status.as_str())
            {
                Some("ok") => theme::success(),
                Some("warn" | "warning") => theme::warning(),
                Some("error") => theme::error(),
                _ => theme::border_unfocused(),
            }
        };

        let status_text = |sub: &str| -> &'static str {
            match self
                .health
                .iter()
                .find(|h| h.subsystem == sub)
                .map(|h| h.status.as_str())
            {
                Some("ok") => "ok",
                Some("warn" | "warning") => "warn",
                Some("error") => "err",
                _ => "─",
            }
        };

        let subsystems = ["wan", "www", "wlan", "lan", "vpn"];
        let mut primary = vec![Span::raw(" ")];
        for (i, &sub) in subsystems.iter().take(3).enumerate() {
            if i > 0 {
                primary.push(Span::raw("  "));
            }
            let dot_color = status_color(sub);
            primary.push(Span::styled(
                sub.to_uppercase(),
                Style::default().fg(theme::text_secondary()),
            ));
            primary.push(Span::styled(" ●", Style::default().fg(dot_color)));
            primary.push(Span::styled(
                format!(" {}", status_text(sub)),
                Style::default().fg(dot_color),
            ));
        }
        lines.push(Line::from(primary));

        let mut secondary = vec![Span::raw(" ")];
        for (i, &sub) in subsystems.iter().skip(3).enumerate() {
            if i > 0 {
                secondary.push(Span::raw("  "));
            }
            let dot_color = status_color(sub);
            secondary.push(Span::styled(
                sub.to_uppercase(),
                Style::default().fg(theme::text_secondary()),
            ));
            secondary.push(Span::styled(" ●", Style::default().fg(dot_color)));
            secondary.push(Span::styled(
                format!(" {}", status_text(sub)),
                Style::default().fg(dot_color),
            ));
        }
        lines.push(Line::from(secondary));
        lines.push(Line::from(" "));

        let wan_link = self.health.iter().find(|h| h.subsystem == "wan");
        let wifi_link = self.health.iter().find(|h| h.subsystem == "wlan");
        let wired_link = self.health.iter().find(|h| h.subsystem == "lan");

        let wan_tx = wan_link.and_then(|h| h.tx_bytes_r).unwrap_or(0);
        let wan_rx = wan_link.and_then(|h| h.rx_bytes_r).unwrap_or(0);
        let wlan_tx = wifi_link.and_then(|h| h.tx_bytes_r).unwrap_or(0);
        let wlan_rx = wifi_link.and_then(|h| h.rx_bytes_r).unwrap_or(0);
        let lan_tx = wired_link.and_then(|h| h.tx_bytes_r).unwrap_or(0);
        let lan_rx = wired_link.and_then(|h| h.rx_bytes_r).unwrap_or(0);

        let link_totals = [
            ("wan", "WAN", wan_tx.saturating_add(wan_rx)),
            ("wlan", "WLAN", wlan_tx.saturating_add(wlan_rx)),
            ("lan", "LAN", lan_tx.saturating_add(lan_rx)),
        ];
        let max_total = link_totals
            .iter()
            .map(|(_, _, total)| *total)
            .max()
            .unwrap_or(0);

        let mut push_link_bar = |sub: &str, label: &str, total: u64| {
            let bar = bytes_fmt::fmt_traffic_bar(total, max_total, bar_width);
            let rate = if total > 0 {
                fmt_rate_compact(total)
            } else {
                "─".to_owned()
            };
            lines.push(Line::from(vec![
                Span::styled(
                    format!(" {label:<5}"),
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(bar, Style::default().fg(status_color(sub))),
                Span::raw(" "),
                Span::styled(
                    truncate_text(&rate, content_w.saturating_sub(8 + usize::from(bar_width))),
                    Style::default()
                        .fg(theme::accent_secondary())
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        };

        for (sub, label, total) in link_totals {
            push_link_bar(sub, label, total);
        }

        let total_tx = wan_tx.saturating_add(wlan_tx).saturating_add(lan_tx);
        let total_rx = wan_rx.saturating_add(wlan_rx).saturating_add(lan_rx);
        let aggregate = format!(
            "↑{}  ↓{}",
            fmt_rate_compact(total_tx),
            fmt_rate_compact(total_rx)
        );
        lines.push(Line::from(vec![
            Span::styled(" Total", Style::default().fg(theme::text_secondary())),
            Span::styled(
                format!(" {}", truncate_text(&aggregate, content_w)),
                Style::default().fg(theme::info()),
            ),
        ]));

        let wlan_ap_count = wifi_link.and_then(|h| h.num_adopted).unwrap_or(0);
        let lan_sw_count = wired_link.and_then(|h| h.num_adopted).unwrap_or(0);
        let wireless_clients = self
            .clients
            .iter()
            .filter(|c| c.client_type == ClientType::Wireless)
            .count();
        let wired_clients = self
            .clients
            .iter()
            .filter(|c| c.client_type == ClientType::Wired)
            .count();

        let infra = truncate_text(
            &format!("AP {wlan_ap_count} · SW {lan_sw_count}"),
            content_w,
        );
        let clients = truncate_text(
            &format!("WiFi {wireless_clients} · Wired {wired_clients}"),
            content_w,
        );
        lines.push(Line::from(vec![
            Span::styled(" Infra", Style::default().fg(theme::text_secondary())),
            Span::styled(
                format!(" {infra}"),
                Style::default().fg(theme::accent_secondary()),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled(" Cli  ", Style::default().fg(theme::text_secondary())),
            Span::styled(
                format!(" {clients}"),
                Style::default().fg(theme::accent_secondary()),
            ),
        ]));

        frame.render_widget(Paragraph::new(lines), inner);
    }
}
