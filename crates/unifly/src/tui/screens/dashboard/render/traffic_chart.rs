use ratatui::layout::{Constraint, Layout};
use ratatui::style::Style;
use ratatui::symbols::Marker;
use ratatui::text::{Line, Span};
use ratatui::widgets::canvas::{Canvas, Line as CanvasLine};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use crate::tui::theme;
use crate::tui::widgets::{bytes_fmt, chart};

use super::super::DashboardScreen;
use super::super::{
    BANDWIDTH_GUTTER_WIDTH, BANDWIDTH_LABEL_WIDTH, BANDWIDTH_TICK_COUNT, LIVE_CHART_WINDOW_SAMPLES,
    MIN_BANDWIDTH_SCALE,
};

impl DashboardScreen {
    /// Hero panel: WAN traffic chart with area fill and Braille line overlay.
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::as_conversions,
        clippy::too_many_lines
    )]
    pub(super) fn render_traffic_chart(
        &self,
        frame: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
    ) {
        let (current_tx, current_rx) = self
            .current_bandwidth()
            .or_else(|| {
                Some((
                    self.bandwidth_tx.last().map(|&(_, value)| value as u64)?,
                    self.bandwidth_rx.last().map(|&(_, value)| value as u64)?,
                ))
            })
            .unwrap_or((0, 0));

        let title = Line::from(vec![
            Span::styled(" WAN Traffic ", theme::title_style()),
            Span::styled("── ", Style::default().fg(theme::border_unfocused())),
            Span::styled(
                format!("TX {} ↑", bytes_fmt::fmt_rate(current_tx)),
                Style::default().fg(theme::accent_secondary()),
            ),
            Span::styled("  ", Style::default()),
            Span::styled(
                format!("RX {} ↓", bytes_fmt::fmt_rate(current_rx)),
                Style::default().fg(theme::accent_tertiary()),
            ),
            Span::styled(
                format!(
                    "  Peak {} ",
                    bytes_fmt::fmt_rate(self.peak_rx.max(self.peak_tx))
                ),
                Style::default().fg(theme::border_unfocused()),
            ),
        ]);

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_default());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.bandwidth_tx.is_empty() {
            frame.render_widget(
                Paragraph::new("  Waiting for data…")
                    .style(Style::default().fg(theme::border_unfocused())),
                inner,
            );
            return;
        }

        #[allow(clippy::cast_precision_loss, clippy::as_conversions)]
        let window_span = LIVE_CHART_WINDOW_SAMPLES.saturating_sub(1) as f64;
        let x_max = self.sample_counter.max(0.0);
        let x_min = x_max - window_span;
        let y_max = self.chart_y_max.max(MIN_BANDWIDTH_SCALE);
        let axis_style = Style::default().fg(theme::border_unfocused());
        let chart_layout = Layout::horizontal([
            Constraint::Length(BANDWIDTH_GUTTER_WIDTH),
            Constraint::Min(1),
        ])
        .split(inner);
        let gutter_area = chart_layout[0];
        let plot_area = chart_layout[1];

        let y_labels = chart::rate_axis_labels(
            y_max,
            BANDWIDTH_TICK_COUNT,
            BANDWIDTH_LABEL_WIDTH,
            axis_style,
        );
        let label_steps = BANDWIDTH_TICK_COUNT.saturating_sub(1).max(1);
        for (idx, label) in y_labels.iter().rev().enumerate() {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let y_offset = if label_steps == 0 {
                0
            } else {
                let rows = plot_area.height.saturating_sub(1);
                (u32::from(rows) * idx as u32 / label_steps as u32) as u16
            };
            let label_area = ratatui::layout::Rect {
                x: gutter_area.x,
                y: plot_area.y + y_offset,
                width: gutter_area.width,
                height: 1,
            };
            frame.render_widget(Paragraph::new(Line::from(label.clone())), label_area);
        }

        let plot_density = (usize::from(plot_area.width.max(1)) * 4).max(160);
        let rx_path = chart::interpolate_fill(&self.bandwidth_rx, plot_density);
        let tx_path = chart::interpolate_fill(&self.bandwidth_tx, plot_density);

        let canvas = Canvas::default()
            .background_color(theme::bg_base())
            .marker(Marker::Octant)
            .x_bounds([x_min, x_max])
            .y_bounds([0.0, y_max])
            .paint(|ctx| {
                ctx.draw(&CanvasLine {
                    x1: x_min,
                    y1: 0.0,
                    x2: x_max,
                    y2: 0.0,
                    color: theme::border_unfocused(),
                });

                for &(x, y) in &rx_path {
                    ctx.draw(&CanvasLine {
                        x1: x,
                        y1: 0.0,
                        x2: x,
                        y2: y,
                        color: theme::rx_fill(),
                    });
                }
                for &(x, y) in &tx_path {
                    ctx.draw(&CanvasLine {
                        x1: x,
                        y1: 0.0,
                        x2: x,
                        y2: y,
                        color: theme::tx_fill(),
                    });
                }

                ctx.layer();

                for pair in rx_path.windows(2) {
                    let [(x1, y1), (x2, y2)] = pair else {
                        continue;
                    };
                    ctx.draw(&CanvasLine {
                        x1: *x1,
                        y1: *y1,
                        x2: *x2,
                        y2: *y2,
                        color: theme::accent_tertiary(),
                    });
                }
                for pair in tx_path.windows(2) {
                    let [(x1, y1), (x2, y2)] = pair else {
                        continue;
                    };
                    ctx.draw(&CanvasLine {
                        x1: *x1,
                        y1: *y1,
                        x2: *x2,
                        y2: *y2,
                        color: theme::accent_secondary(),
                    });
                }
            });

        frame.render_widget(canvas, plot_area);
    }
}
