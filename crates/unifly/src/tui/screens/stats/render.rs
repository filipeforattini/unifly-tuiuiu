use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::symbols::Marker;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, Paragraph};

use super::{
    BANDWIDTH_LABEL_WIDTH, BANDWIDTH_TICK_COUNT, CLIENT_LABEL_WIDTH, CLIENT_TICK_COUNT,
    MIN_BANDWIDTH_SCALE, StatsScreen,
};
use crate::tui::theme;
use crate::tui::widgets::{bytes_fmt, chart, sub_tabs};

impl StatsScreen {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::as_conversions
    )]
    pub(super) fn render_bandwidth_chart(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" WAN Bandwidth ")
            .title_style(theme::title_style())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_default());

        if self.bandwidth_tx.is_empty() && self.bandwidth_rx.is_empty() {
            let inner = block.inner(area);
            frame.render_widget(block, area);
            frame.render_widget(
                Paragraph::new("  No bandwidth data yet")
                    .style(Style::default().fg(theme::border_unfocused())),
                inner,
            );
            return;
        }

        let x_min = self
            .bandwidth_tx
            .first()
            .map_or(f64::MAX, |&(x, _)| x)
            .min(self.bandwidth_rx.first().map_or(f64::MAX, |&(x, _)| x));
        let x_max = self
            .bandwidth_tx
            .last()
            .map_or(0.0, |&(x, _)| x)
            .max(self.bandwidth_rx.last().map_or(0.0, |&(x, _)| x));
        let (x_min, x_max) = if (x_max - x_min).abs() < f64::EPSILON {
            (x_min - 0.5, x_max + 0.5)
        } else {
            (x_min, x_max)
        };
        let y_max = self.bandwidth_y_max.max(MIN_BANDWIDTH_SCALE);

        let fill_density = (usize::from(area.width.saturating_sub(8)) * 3).max(120);
        let rx_fill_data = chart::interpolate_fill(&self.bandwidth_rx, fill_density);
        let tx_fill_data = chart::interpolate_fill(&self.bandwidth_tx, fill_density);

        let rx_fill = Dataset::default()
            .marker(Marker::HalfBlock)
            .graph_type(GraphType::Bar)
            .style(Style::default().fg(theme::rx_fill()))
            .data(&rx_fill_data);

        let tx_fill = Dataset::default()
            .marker(Marker::HalfBlock)
            .graph_type(GraphType::Bar)
            .style(Style::default().fg(theme::tx_fill()))
            .data(&tx_fill_data);

        let tx_line = Dataset::default()
            .name("TX")
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(theme::accent_secondary()))
            .data(&self.bandwidth_tx);

        let rx_line = Dataset::default()
            .name("RX")
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(theme::accent_tertiary()))
            .data(&self.bandwidth_rx);

        let axis_style = Style::default().fg(theme::border_unfocused());
        let y_labels = chart::rate_axis_labels(
            y_max,
            BANDWIDTH_TICK_COUNT,
            BANDWIDTH_LABEL_WIDTH,
            axis_style,
        );

        let chart = Chart::new(vec![rx_fill, tx_fill, tx_line, rx_line])
            .block(block)
            .x_axis(Axis::default().bounds([x_min, x_max]).style(axis_style))
            .y_axis(
                Axis::default()
                    .bounds([0.0, y_max])
                    .labels(y_labels)
                    .style(axis_style),
            );

        frame.render_widget(chart, area);
    }

    pub(super) fn render_client_chart(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Client Count ")
            .title_style(theme::title_style())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_default());

        if self.client_counts.is_empty() {
            let inner = block.inner(area);
            frame.render_widget(block, area);
            frame.render_widget(
                Paragraph::new("  No client data yet")
                    .style(Style::default().fg(theme::border_unfocused())),
                inner,
            );
            return;
        }

        let x_min = self.client_counts.first().map_or(0.0, |(x, _)| *x);
        let x_max = self.client_counts.last().map_or(1.0, |(x, _)| *x);
        let (x_min, x_max) = if (x_max - x_min).abs() < f64::EPSILON {
            (x_min - 0.5, x_max + 0.5)
        } else {
            (x_min, x_max)
        };
        let y_max = self.client_y_max.max(1.0);

        let dataset = Dataset::default()
            .name("Clients")
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(theme::accent_primary()))
            .data(&self.client_counts);

        let axis_style = Style::default().fg(theme::border_unfocused());
        let y_labels =
            chart::count_axis_labels(y_max, CLIENT_TICK_COUNT, CLIENT_LABEL_WIDTH, axis_style);
        let chart = Chart::new(vec![dataset])
            .block(block)
            .x_axis(Axis::default().style(axis_style).bounds([x_min, x_max]))
            .y_axis(
                Axis::default()
                    .style(axis_style)
                    .bounds([0.0, y_max])
                    .labels(y_labels),
            );

        frame.render_widget(chart, area);
    }

    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::as_conversions
    )]
    pub(super) fn render_top_apps(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Top Applications ")
            .title_style(theme::title_style())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_default());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.dpi_apps.is_empty() {
            frame.render_widget(
                Paragraph::new("  No DPI data available")
                    .style(Style::default().fg(theme::border_unfocused())),
                inner,
            );
            return;
        }

        let max_rows = inner.height as usize;
        let bar_budget = inner.width.saturating_sub(26) as usize;
        let colors = theme::chart_series();
        let max_bytes = self.dpi_apps.first().map_or(1, |(_, bytes)| *bytes).max(1);

        let mut lines = Vec::new();
        for (index, (name, bytes)) in self.dpi_apps.iter().enumerate().take(max_rows) {
            let fraction = *bytes as f64 / max_bytes as f64;
            let bar_width = (fraction * bar_budget as f64).round().max(1.0) as usize;
            let bar: String = "█".repeat(bar_width.min(bar_budget));
            let color = colors[index % colors.len()];
            let display_name: String = name.chars().take(14).collect();
            let bytes_str = bytes_fmt::fmt_bytes_short(*bytes);

            lines.push(Line::from(vec![
                Span::styled(
                    format!("  {display_name:<14} "),
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(bar, Style::default().fg(color)),
                Span::styled(
                    format!(" {bytes_str:>6}"),
                    Style::default().fg(theme::text_secondary()),
                ),
            ]));
        }

        frame.render_widget(Paragraph::new(lines), inner);
    }

    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::as_conversions
    )]
    pub(super) fn render_categories(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Traffic by Category ")
            .title_style(theme::title_style())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_default());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.dpi_categories.is_empty() {
            frame.render_widget(
                Paragraph::new("  No category data")
                    .style(Style::default().fg(theme::border_unfocused())),
                inner,
            );
            return;
        }

        let max_rows = inner.height as usize;
        let bar_budget = inner.width.saturating_sub(22) as usize;
        let colors = theme::chart_series();
        let total_bytes: u64 = self.dpi_categories.iter().map(|(_, bytes)| *bytes).sum();

        let mut lines = Vec::new();
        for (index, (name, bytes)) in self.dpi_categories.iter().enumerate().take(max_rows) {
            let pct = if total_bytes > 0 {
                *bytes as f64 / total_bytes as f64 * 100.0
            } else {
                0.0
            };
            let bar_width = (pct / 100.0 * bar_budget as f64).round().max(0.0) as usize;
            let bar: String = "█".repeat(bar_width.min(bar_budget));
            let color = colors[index % colors.len()];
            let display_name: String = name.chars().take(12).collect();

            lines.push(Line::from(vec![
                Span::styled(
                    format!("  {display_name:<12} "),
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(bar, Style::default().fg(color)),
                Span::styled(
                    format!(" {pct:>4.0}%"),
                    Style::default().fg(theme::text_secondary()),
                ),
            ]));
        }

        frame.render_widget(Paragraph::new(lines), inner);
    }

    pub(super) fn render_screen(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Statistics ")
            .title_style(theme::title_style())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(if self.focused {
                theme::border_focused()
            } else {
                theme::border_default()
            });

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Percentage(45),
            Constraint::Min(8),
            Constraint::Length(1),
        ])
        .split(inner);

        let period_line =
            sub_tabs::render_sub_tabs(&["1h", "24h", "7d", "30d"], self.period_index());
        frame.render_widget(Paragraph::new(period_line), layout[0]);

        self.render_bandwidth_chart(frame, layout[1]);

        let bottom = Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(layout[2]);
        let left_col = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(bottom[0]);

        self.render_client_chart(frame, left_col[0]);
        self.render_categories(frame, left_col[1]);
        self.render_top_apps(frame, bottom[1]);

        let hints = Line::from(vec![
            Span::styled("  h ", theme::key_hint_key()),
            Span::styled("1h  ", theme::key_hint()),
            Span::styled("d ", theme::key_hint_key()),
            Span::styled("24h  ", theme::key_hint()),
            Span::styled("w ", theme::key_hint_key()),
            Span::styled("7d  ", theme::key_hint()),
            Span::styled("m ", theme::key_hint_key()),
            Span::styled("30d  ", theme::key_hint()),
            Span::styled("r ", theme::key_hint_key()),
            Span::styled("refresh", theme::key_hint()),
        ]);
        frame.render_widget(Paragraph::new(hints), layout[3]);
    }
}
