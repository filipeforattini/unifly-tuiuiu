use std::sync::Arc;

use super::DevicesScreen;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table};

use unifly_api::{Device, DeviceState};

use crate::tui::action::DeviceDetailTab;
use crate::tui::theme;
use crate::tui::widgets::{bytes_fmt, status_indicator, sub_tabs};

impl DevicesScreen {
    pub(super) fn render_screen(&self, frame: &mut Frame, area: Rect) {
        let filtered = self.filtered_devices();
        let selected_index = self.selected_row_index(&filtered);
        let total = self.devices.len();
        let shown = filtered.len();
        let title = if self.search_query.is_empty() {
            format!(" Devices ({total}) ")
        } else {
            format!(" Devices ({shown}/{total}) ")
        };

        let block = Block::default()
            .title(title)
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

        let (table_area, detail_area) = if self.detail_open {
            let chunks = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(inner);
            (chunks[0], Some(chunks[1]))
        } else {
            (inner, None)
        };

        let header_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(table_area);

        let filter_text = if self.search_query.is_empty() {
            Span::styled("[all]", Style::default().fg(theme::accent_secondary()))
        } else {
            Span::styled(
                format!("[\"{}\" ]", self.search_query),
                Style::default().fg(theme::warning()),
            )
        };
        let filter_line = Line::from(vec![
            Span::styled(" Filter: ", Style::default().fg(theme::text_secondary())),
            filter_text,
            Span::styled("  Sort: ", Style::default().fg(theme::text_secondary())),
            Span::styled("[name ↑]", Style::default().fg(theme::accent_secondary())),
            Span::styled(
                format!("  {:>width$}", format!("{shown} devices"), width = 20),
                Style::default().fg(theme::text_secondary()),
            ),
        ]);
        frame.render_widget(Paragraph::new(filter_line), header_layout[0]);

        let header = Row::new(vec![
            Cell::from("Status").style(theme::table_header()),
            Cell::from("Name").style(theme::table_header()),
            Cell::from("Model").style(theme::table_header()),
            Cell::from("IP").style(theme::table_header()),
            Cell::from("CPU").style(theme::table_header()),
            Cell::from("Mem").style(theme::table_header()),
            Cell::from("TX/RX").style(theme::table_header()),
            Cell::from("Uptime").style(theme::table_header()),
        ]);

        let rows: Vec<Row> = filtered
            .iter()
            .enumerate()
            .map(|(index, device)| self.render_table_row(index, selected_index, device))
            .collect();

        let widths = [
            Constraint::Length(3),
            Constraint::Min(14),
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Length(7),
            Constraint::Length(7),
            Constraint::Length(11),
            Constraint::Length(8),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .row_highlight_style(theme::table_selected());

        let mut state = self.table_state;
        frame.render_stateful_widget(table, header_layout[1], &mut state);

        let hints = Line::from(vec![
            Span::styled("  j/k ", theme::key_hint_key()),
            Span::styled("navigate  ", theme::key_hint()),
            Span::styled("Enter ", theme::key_hint_key()),
            Span::styled("detail  ", theme::key_hint()),
            Span::styled("R ", theme::key_hint_key()),
            Span::styled("restart  ", theme::key_hint()),
            Span::styled("L ", theme::key_hint_key()),
            Span::styled("locate", theme::key_hint()),
        ]);
        frame.render_widget(Paragraph::new(hints), header_layout[2]);

        if let Some(detail_area) = detail_area
            && let Some(device) = self.detail_device()
        {
            self.render_detail(frame, detail_area, device);
        }
    }

    fn render_table_row(
        &self,
        index: usize,
        selected_index: Option<usize>,
        device: &Arc<Device>,
    ) -> Row<'static> {
        let is_selected = Some(index) == selected_index;
        let prefix = if is_selected { "▸" } else { " " };
        let status = status_indicator::status_char(device.state);
        let name = device.name.as_deref().unwrap_or("Unknown");
        let model = device.model.as_deref().unwrap_or("─");
        let ip = device.ip.map_or_else(|| "─".into(), |ip| ip.to_string());
        let cpu = device
            .stats
            .cpu_utilization_pct
            .map_or_else(|| "·····".into(), |value| format!("{value:.0}%"));
        let mem = device
            .stats
            .memory_utilization_pct
            .map_or_else(|| "·····".into(), |value| format!("{value:.0}%"));
        let traffic = device.stats.uplink_bandwidth.as_ref().map_or_else(
            || "···/···".into(),
            |bandwidth| {
                bytes_fmt::fmt_tx_rx(bandwidth.tx_bytes_per_sec, bandwidth.rx_bytes_per_sec)
            },
        );
        let uptime = device
            .stats
            .uptime_secs
            .map_or_else(|| "···".into(), bytes_fmt::fmt_uptime);

        let status_color = match device.state {
            DeviceState::Online => theme::success(),
            DeviceState::Offline | DeviceState::ConnectionInterrupted | DeviceState::Isolated => {
                theme::error()
            }
            DeviceState::PendingAdoption => theme::accent_primary(),
            _ => theme::warning(),
        };

        let row_style = if is_selected {
            theme::table_selected()
        } else {
            theme::table_row()
        };

        Row::new(vec![
            Cell::from(format!("{prefix}{status}")).style(Style::default().fg(status_color)),
            Cell::from(name.to_string()).style(
                Style::default()
                    .fg(theme::accent_secondary())
                    .add_modifier(if is_selected {
                        Modifier::BOLD
                    } else {
                        Modifier::empty()
                    }),
            ),
            Cell::from(model.to_string()),
            Cell::from(ip).style(Style::default().fg(theme::accent_tertiary())),
            Cell::from(cpu),
            Cell::from(mem),
            Cell::from(traffic),
            Cell::from(uptime),
        ])
        .style(row_style)
    }

    fn render_detail(&self, frame: &mut Frame, area: Rect, device: &Device) {
        let name = device.name.as_deref().unwrap_or("Unknown");
        let model = device.model.as_deref().unwrap_or("─");
        let ip = device
            .ip
            .map_or_else(|| "─".into(), |device_ip| device_ip.to_string());
        let mac = device.mac.to_string();

        let title = format!(" {name}  ·  {model}  ·  {ip}  ·  {mac} ");
        let block = Block::default()
            .title(title)
            .title_style(theme::title_style())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_focused());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let tabs_layout = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(inner);

        let tab_labels = &["Overview", "Performance", "Radios", "Clients", "Ports"];
        let active_idx = match self.detail_tab {
            DeviceDetailTab::Overview => 0,
            DeviceDetailTab::Performance => 1,
            DeviceDetailTab::Radios => 2,
            DeviceDetailTab::Clients => 3,
            DeviceDetailTab::Ports => 4,
        };
        let tab_line = sub_tabs::render_sub_tabs(tab_labels, active_idx);
        frame.render_widget(
            Paragraph::new(vec![Line::from(""), tab_line]),
            tabs_layout[0],
        );

        match self.detail_tab {
            DeviceDetailTab::Overview => self.render_overview_tab(frame, tabs_layout[1], device),
            DeviceDetailTab::Performance => {
                self.render_performance_tab(frame, tabs_layout[1], device);
            }
            DeviceDetailTab::Radios => self.render_radios_tab(frame, tabs_layout[1], device),
            DeviceDetailTab::Clients => {
                let text = format!("  Connected clients: {}", device.client_count.unwrap_or(0));
                frame.render_widget(
                    Paragraph::new(text).style(theme::table_row()),
                    tabs_layout[1],
                );
            }
            DeviceDetailTab::Ports => self.render_ports_tab(frame, tabs_layout[1], device),
        }

        let hints = Line::from(vec![
            Span::styled("  h/l ", theme::key_hint_key()),
            Span::styled("switch tabs  ", theme::key_hint()),
            Span::styled("R ", theme::key_hint_key()),
            Span::styled("restart  ", theme::key_hint()),
            Span::styled("L ", theme::key_hint_key()),
            Span::styled("locate  ", theme::key_hint()),
            Span::styled("Esc ", theme::key_hint_key()),
            Span::styled("back", theme::key_hint()),
        ]);
        frame.render_widget(Paragraph::new(hints), tabs_layout[2]);
    }

    fn render_overview_tab(&self, frame: &mut Frame, area: Rect, device: &Device) {
        let state_span = status_indicator::status_span(device.state);
        let state_label = format!("{:?}", device.state);
        let firmware = device.firmware_version.as_deref().unwrap_or("─");
        let fw_status = if device.firmware_updatable {
            "update available"
        } else {
            "up to date"
        };
        let uptime = device
            .stats
            .uptime_secs
            .map_or_else(|| "─".into(), bytes_fmt::fmt_uptime);
        let adopted = device.adopted_at.map_or_else(
            || "─".into(),
            |dt| dt.format("%Y-%m-%d %H:%M UTC").to_string(),
        );

        let lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "  State          ",
                    Style::default().fg(theme::text_secondary()),
                ),
                state_span,
                Span::styled(
                    format!(" {state_label}"),
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(
                    "       Adopted     ",
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(adopted, Style::default().fg(theme::accent_secondary())),
            ]),
            Line::from(vec![
                Span::styled(
                    "  Firmware       ",
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(
                    format!("{firmware} ({fw_status})"),
                    Style::default().fg(theme::accent_secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "  Uptime         ",
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(uptime, Style::default().fg(theme::accent_secondary())),
            ]),
            Line::from(vec![
                Span::styled(
                    "  MAC            ",
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(
                    device.mac.to_string(),
                    Style::default().fg(theme::accent_tertiary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "  Type           ",
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(
                    format!("{:?}", device.device_type),
                    Style::default().fg(theme::text_secondary()),
                ),
            ]),
        ];

        frame.render_widget(Paragraph::new(lines), area);
    }

    fn render_performance_tab(&self, frame: &mut Frame, area: Rect, device: &Device) {
        let cpu = device
            .stats
            .cpu_utilization_pct
            .map_or_else(|| "─".into(), |value| format!("{value:.1}%"));
        let mem = device
            .stats
            .memory_utilization_pct
            .map_or_else(|| "─".into(), |value| format!("{value:.1}%"));
        let load = device
            .stats
            .load_average_1m
            .map_or_else(|| "─".into(), |value| format!("{value:.2}"));

        let cpu_color = device
            .stats
            .cpu_utilization_pct
            .map_or(theme::text_secondary(), |value| {
                if value < 50.0 {
                    theme::success()
                } else if value < 75.0 {
                    theme::accent_secondary()
                } else if value < 90.0 {
                    theme::warning()
                } else {
                    theme::error()
                }
            });

        let mem_color =
            device
                .stats
                .memory_utilization_pct
                .map_or(theme::text_secondary(), |value| {
                    if value < 50.0 {
                        theme::success()
                    } else if value < 75.0 {
                        theme::accent_secondary()
                    } else if value < 90.0 {
                        theme::warning()
                    } else {
                        theme::error()
                    }
                });

        let lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  CPU     ", Style::default().fg(theme::text_secondary())),
                Span::styled(cpu, Style::default().fg(cpu_color)),
            ]),
            Line::from(vec![
                Span::styled("  Memory  ", Style::default().fg(theme::text_secondary())),
                Span::styled(mem, Style::default().fg(mem_color)),
            ]),
            Line::from(vec![
                Span::styled("  Load    ", Style::default().fg(theme::text_secondary())),
                Span::styled(load, Style::default().fg(theme::text_secondary())),
            ]),
        ];

        frame.render_widget(Paragraph::new(lines), area);
    }

    fn render_radios_tab(&self, frame: &mut Frame, area: Rect, device: &Device) {
        let mut lines = vec![Line::from("")];

        if device.radios.is_empty() {
            lines.push(Line::from(Span::styled(
                "  No radio data available",
                Style::default().fg(theme::border_unfocused()),
            )));
        } else {
            for radio in &device.radios {
                let freq = format!("{:.1} GHz", radio.frequency_ghz);
                let channel = radio
                    .channel
                    .map_or_else(|| "─".into(), |value| format!("ch {value}"));
                let width = radio
                    .channel_width_mhz
                    .map_or_else(|| "─".into(), |value| format!("{value} MHz"));

                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {freq:<10}"),
                        Style::default()
                            .fg(theme::accent_secondary())
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{channel:<8} {width}"),
                        Style::default().fg(theme::text_secondary()),
                    ),
                ]));
            }
        }

        frame.render_widget(Paragraph::new(lines), area);
    }

    fn render_ports_tab(&self, frame: &mut Frame, area: Rect, device: &Device) {
        let mut lines = vec![Line::from("")];

        if device.ports.is_empty() {
            lines.push(Line::from(Span::styled(
                "  No port data available",
                Style::default().fg(theme::border_unfocused()),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                "  Port  State   Speed      PoE",
                theme::table_header(),
            )));

            for port in &device.ports {
                let idx_str = port.index.to_string();
                let name = port.name.as_deref().unwrap_or(&idx_str);
                let state_color = match port.state {
                    unifly_api::model::PortState::Up => theme::success(),
                    unifly_api::model::PortState::Down => theme::error(),
                    unifly_api::model::PortState::Unknown => theme::text_secondary(),
                };
                let state_str = format!("{:?}", port.state);
                let speed = port.speed_mbps.map_or_else(
                    || "─".into(),
                    |value| {
                        if value >= 1000 {
                            format!("{}G", value / 1000)
                        } else {
                            format!("{value}M")
                        }
                    },
                );
                let poe = port
                    .poe
                    .as_ref()
                    .map_or("─", |poe| if poe.enabled { "✓" } else { "✗" });

                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {name:<6}"),
                        Style::default().fg(theme::accent_secondary()),
                    ),
                    Span::styled(format!("{state_str:<8}"), Style::default().fg(state_color)),
                    Span::styled(
                        format!("{speed:<11}"),
                        Style::default().fg(theme::text_secondary()),
                    ),
                    Span::styled(poe, Style::default().fg(theme::text_secondary())),
                ]));
            }
        }

        frame.render_widget(Paragraph::new(lines), area);
    }
}
