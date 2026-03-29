use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use unifly_api::{ClientType, DeviceType};

use crate::tui::theme;
use crate::tui::widgets::bytes_fmt;

use super::super::DashboardScreen;
use super::super::helpers::truncate_text;

impl DashboardScreen {
    /// Capacity card - CPU/MEM bars, load averages, and fleet counts.
    #[allow(clippy::cast_possible_truncation)]
    pub(super) fn render_capacity(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let block = Block::default()
            .title(Span::styled(" Capacity ", theme::title_style()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_default());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let gateway = self
            .devices
            .iter()
            .find(|d| d.device_type == DeviceType::Gateway);
        let w = usize::from(inner.width);
        let content_w = w.saturating_sub(7);
        let bar_width = u16::try_from(content_w.saturating_sub(9).clamp(6, 18)).unwrap_or(6);

        let pct_bar_color = |pct: f64| -> ratatui::style::Color {
            if pct > 80.0 {
                theme::error()
            } else if pct > 50.0 {
                theme::warning()
            } else {
                theme::accent_secondary()
            }
        };

        let mut lines = Vec::new();
        let mut push_bar = |label: &str, pct: Option<f64>| {
            let line = if let Some(pct) = pct {
                let (filled, empty) = bytes_fmt::fmt_pct_bar(pct, bar_width);
                Line::from(vec![
                    Span::styled(
                        format!(" {label:<4}"),
                        Style::default().fg(theme::text_secondary()),
                    ),
                    Span::styled(filled, Style::default().fg(pct_bar_color(pct))),
                    Span::styled(empty, Style::default().fg(theme::border_unfocused())),
                    Span::styled(
                        format!(" {pct:>5.1}%"),
                        Style::default().fg(theme::text_secondary()),
                    ),
                ])
            } else {
                Line::from(vec![
                    Span::styled(
                        format!(" {label:<4}"),
                        Style::default().fg(theme::text_secondary()),
                    ),
                    Span::styled("─", Style::default().fg(theme::border_unfocused())),
                ])
            };
            lines.push(line);
        };

        push_bar("CPU", gateway.and_then(|g| g.stats.cpu_utilization_pct));
        push_bar("MEM", gateway.and_then(|g| g.stats.memory_utilization_pct));
        lines.push(Line::from(" "));

        if let Some(gw) = gateway
            && let (Some(l1), Some(l5), Some(l15)) = (
                gw.stats.load_average_1m,
                gw.stats.load_average_5m,
                gw.stats.load_average_15m,
            )
        {
            let load = truncate_text(&format!("{l1:.2} / {l5:.2} / {l15:.2}"), content_w);
            lines.push(Line::from(vec![
                Span::styled(" Load ", Style::default().fg(theme::text_secondary())),
                Span::styled(load, Style::default().fg(theme::accent_secondary())),
            ]));
        }

        let total_devices = self.devices.len();
        let online = self
            .devices
            .iter()
            .filter(|d| d.state == unifly_api::DeviceState::Online)
            .count();
        let total_clients = self.clients.len();
        let wireless = self
            .clients
            .iter()
            .filter(|c| c.client_type == ClientType::Wireless)
            .count();
        let wired = self
            .clients
            .iter()
            .filter(|c| c.client_type == ClientType::Wired)
            .count();

        let dev_summary = truncate_text(
            &format!("{total_devices} total · {online} online"),
            content_w,
        );
        let cli_summary = truncate_text(
            &format!("{total_clients} total · {wireless} wifi · {wired} wired"),
            content_w,
        );

        lines.push(Line::from(vec![
            Span::styled(" Dev  ", Style::default().fg(theme::text_secondary())),
            Span::styled(dev_summary, Style::default().fg(theme::accent_secondary())),
        ]));
        lines.push(Line::from(vec![
            Span::styled(" Cli  ", Style::default().fg(theme::text_secondary())),
            Span::styled(cli_summary, Style::default().fg(theme::accent_secondary())),
        ]));

        frame.render_widget(Paragraph::new(lines), inner);
    }
}
