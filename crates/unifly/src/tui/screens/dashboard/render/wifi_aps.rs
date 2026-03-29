use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use unifly_api::DeviceType;

use crate::tui::theme;

use super::super::DashboardScreen;

impl DashboardScreen {
    /// WiFi / APs panel - tabular layout with header, aligned columns.
    #[allow(clippy::too_many_lines, clippy::cast_possible_truncation)]
    pub(super) fn render_wifi_aps(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let block = Block::default()
            .title(Span::styled(" WiFi / APs ", theme::title_style()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_default());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let aps: Vec<_> = self
            .devices
            .iter()
            .filter(|d| d.device_type == DeviceType::AccessPoint)
            .collect();

        if aps.is_empty() {
            frame.render_widget(
                Paragraph::new("  No APs").style(Style::default().fg(theme::border_unfocused())),
                inner,
            );
            return;
        }

        let max_rows = usize::from(inner.height);
        let w = usize::from(inner.width);
        let mut lines = Vec::new();

        let has_radios = aps.iter().any(|ap| !ap.radios.is_empty());
        let cli_col = 4_usize;
        let exp_col = 5_usize;
        let fixed_cols = 1 + cli_col + exp_col;
        let remaining = w.saturating_sub(fixed_cols);

        let (name_width, chan_width) = if has_radios {
            let nw = remaining.saturating_sub(1).clamp(6, 16);
            let cw = remaining.saturating_sub(nw + 1);
            (nw, cw)
        } else {
            (remaining.clamp(6, 24), 0)
        };

        let mut hdr = vec![
            Span::styled(
                format!(" {:<name_width$}", "AP"),
                Style::default()
                    .fg(theme::border_unfocused())
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{:>cli_col$}", "Cli"),
                Style::default()
                    .fg(theme::border_unfocused())
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{:>exp_col$}", "Exp"),
                Style::default()
                    .fg(theme::border_unfocused())
                    .add_modifier(Modifier::BOLD),
            ),
        ];
        if chan_width >= 4 {
            hdr.push(Span::styled(
                format!(" {:<chan_width$}", "Chan"),
                Style::default()
                    .fg(theme::border_unfocused())
                    .add_modifier(Modifier::BOLD),
            ));
        }
        lines.push(Line::from(hdr));

        let mut aps_sorted: Vec<_> = aps.iter().collect();
        aps_sorted.sort_by(|a, b| {
            b.client_count
                .unwrap_or(0)
                .cmp(&a.client_count.unwrap_or(0))
        });

        for ap in &aps_sorted {
            if lines.len() >= max_rows {
                break;
            }

            let ap_name: String = ap
                .name
                .as_deref()
                .unwrap_or("AP")
                .chars()
                .take(name_width)
                .collect();
            let cli = ap.client_count.unwrap_or(0);

            let channels: Vec<String> = ap
                .radios
                .iter()
                .map(|r| {
                    let ch = r.channel.map_or_else(|| "─".into(), |c| c.to_string());
                    if r.frequency_ghz >= 5.9 {
                        format!("6G:{ch}")
                    } else if r.frequency_ghz >= 4.9 {
                        format!("5G:{ch}")
                    } else {
                        format!("2G:{ch}")
                    }
                })
                .collect();
            let ch_str: String = channels.join(" ").chars().take(chan_width).collect();

            let satisfaction: Vec<u8> = self
                .clients
                .iter()
                .filter(|c| {
                    c.uplink_device_mac.as_ref() == Some(&ap.mac)
                        || c.wireless
                            .as_ref()
                            .and_then(|wl| wl.bssid.as_ref())
                            .is_some_and(|bssid| *bssid == ap.mac)
                })
                .filter_map(|c| c.wireless.as_ref()?.satisfaction)
                .collect();
            let avg_exp = if satisfaction.is_empty() {
                None
            } else {
                Some(
                    satisfaction.iter().map(|s| u32::from(*s)).sum::<u32>()
                        / u32::try_from(satisfaction.len()).unwrap_or(1),
                )
            };

            let exp_color = |e: u32| -> ratatui::style::Color {
                if e >= 80 {
                    theme::success()
                } else if e >= 50 {
                    theme::warning()
                } else {
                    theme::error()
                }
            };

            let mut spans = vec![
                Span::styled(
                    format!(" {ap_name:<name_width$}"),
                    Style::default()
                        .fg(theme::accent_secondary())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{cli:>cli_col$}"),
                    Style::default().fg(theme::warning()),
                ),
            ];

            if let Some(exp) = avg_exp {
                spans.push(Span::styled(
                    format!("{exp:>4}%"),
                    Style::default().fg(exp_color(exp)),
                ));
            } else {
                spans.push(Span::styled(
                    format!("{:>exp_col$}", "─"),
                    Style::default().fg(theme::border_unfocused()),
                ));
            }

            if chan_width >= 4 {
                spans.push(Span::styled(
                    format!(" {ch_str}"),
                    Style::default().fg(theme::border_unfocused()),
                ));
            }

            lines.push(Line::from(spans));
        }

        frame.render_widget(Paragraph::new(lines), inner);
    }
}
