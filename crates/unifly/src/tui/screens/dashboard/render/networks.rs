use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use unifly_api::model::Ipv6Mode;

use crate::tui::theme;

use super::super::DashboardScreen;

impl DashboardScreen {
    /// Networks panel with IPv6 config.
    #[allow(clippy::cast_possible_truncation, clippy::too_many_lines)]
    pub(super) fn render_networks(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let block = Block::default()
            .title(Span::styled(" Networks ", theme::title_style()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_default());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.networks.is_empty() {
            frame.render_widget(
                Paragraph::new("  No networks")
                    .style(Style::default().fg(theme::border_unfocused())),
                inner,
            );
            return;
        }

        let mut sorted: Vec<_> = self.networks.iter().collect();
        sorted.sort_by_key(|n| n.vlan_id.unwrap_or(0));

        let max_lines = usize::from(inner.height);
        let mut lines = Vec::new();

        for net in &sorted {
            if lines.len() >= max_lines {
                break;
            }

            let name: String = net.name.chars().take(10).collect();
            let vlan = net.vlan_id.map_or_else(|| "─".into(), |v| format!("{v}"));
            let subnet = net.subnet.as_deref().unwrap_or("─");

            let client_count = self
                .clients
                .iter()
                .filter(|c| c.vlan.is_some_and(|v| Some(v) == net.vlan_id))
                .count();
            let client_str = if client_count > 0 {
                format!(" {client_count}c")
            } else {
                String::new()
            };

            lines.push(Line::from(vec![
                Span::styled(
                    format!(" {name:<8}"),
                    Style::default()
                        .fg(theme::accent_secondary())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{vlan:<3}"),
                    Style::default().fg(theme::accent_tertiary()),
                ),
                Span::styled(
                    subnet.to_string(),
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(client_str, Style::default().fg(theme::warning())),
            ]));

            if lines.len() < max_lines && net.ipv6_enabled {
                let mode = match net.ipv6_mode {
                    Some(Ipv6Mode::PrefixDelegation) => "PD",
                    Some(Ipv6Mode::Static) => "Static",
                    Some(_) | None => "On",
                };
                let prefix = net.ipv6_prefix.as_deref().unwrap_or("─");
                let mut extras = Vec::new();
                if net.slaac_enabled {
                    extras.push("SLAAC");
                }
                if net.dhcpv6_enabled {
                    extras.push("DHCPv6");
                }
                let extras_str = if extras.is_empty() {
                    String::new()
                } else {
                    format!(" {}", extras.join("+"))
                };

                lines.push(Line::from(vec![
                    Span::styled(" ⬡ ", Style::default().fg(theme::border_unfocused())),
                    Span::styled(
                        format!("{mode} {prefix}{extras_str}"),
                        Style::default().fg(theme::info()),
                    ),
                ]));
            }
        }

        frame.render_widget(Paragraph::new(lines), inner);
    }
}
