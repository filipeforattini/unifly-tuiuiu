use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use crate::tui::theme;
use crate::tui::widgets::bytes_fmt;

use super::super::DashboardScreen;

impl DashboardScreen {
    /// Top Clients panel with proportional traffic bars.
    #[allow(clippy::cast_possible_truncation)]
    pub(super) fn render_top_clients(
        &self,
        frame: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
    ) {
        let block = Block::default()
            .title(Span::styled(" Top Clients ", theme::title_style()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_default());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let max_rows = usize::from(inner.height);
        let mut sorted: Vec<_> = self.clients.iter().collect();
        sorted.sort_by(|a, b| {
            let a_total = a.tx_bytes.unwrap_or(0) + a.rx_bytes.unwrap_or(0);
            let b_total = b.tx_bytes.unwrap_or(0) + b.rx_bytes.unwrap_or(0);
            b_total.cmp(&a_total)
        });

        let visible: Vec<_> = sorted.iter().take(max_rows.min(8)).collect();

        let max_traffic = visible
            .first()
            .map_or(1, |c| c.tx_bytes.unwrap_or(0) + c.rx_bytes.unwrap_or(0))
            .max(1);

        let traffic_width = 7u16;
        let padding = 3u16;

        let longest_name = visible
            .iter()
            .map(|c| {
                c.name
                    .as_deref()
                    .or(c.hostname.as_deref())
                    .unwrap_or("unknown")
                    .len()
            })
            .max()
            .unwrap_or(8);
        let name_cap = usize::from(inner.width.saturating_sub(traffic_width + padding + 4));
        let name_width = longest_name.min(name_cap).max(8);

        let bar_width = inner.width.saturating_sub(
            u16::try_from(name_width).unwrap_or(u16::MAX) + traffic_width + padding + 1,
        );

        let mut lines = Vec::new();
        for client in &visible {
            let name = client
                .name
                .as_deref()
                .or(client.hostname.as_deref())
                .unwrap_or("unknown");
            let total = client.tx_bytes.unwrap_or(0) + client.rx_bytes.unwrap_or(0);
            let traffic = bytes_fmt::fmt_bytes_short(total);
            let bar = bytes_fmt::fmt_traffic_bar(total, max_traffic, bar_width);

            let display_name: String = name.chars().take(name_width).collect();
            lines.push(Line::from(vec![
                Span::styled(
                    format!(" {display_name:<name_width$}"),
                    Style::default().fg(theme::accent_secondary()),
                ),
                Span::styled(bar, Style::default().fg(theme::accent_primary())),
                Span::styled(
                    format!(" {traffic:>6}"),
                    Style::default().fg(theme::text_secondary()),
                ),
            ]));
        }

        if lines.is_empty() {
            lines.push(Line::from(Span::styled(
                "  No clients",
                Style::default().fg(theme::border_unfocused()),
            )));
        }

        frame.render_widget(Paragraph::new(lines), inner);
    }
}
