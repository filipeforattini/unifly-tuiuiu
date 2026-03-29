use super::ClientsScreen;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table};

use unifly_api::{Client, ClientType};

use crate::tui::theme;
use crate::tui::widgets::{bytes_fmt, sub_tabs};

impl ClientsScreen {
    pub(super) fn render_screen(&self, frame: &mut Frame, area: Rect) {
        let filtered = self.filtered_clients();
        let total = self.clients.len();
        let shown = filtered.len();
        let title = if self.search_query.is_empty() {
            format!(" Clients ({shown}/{total}) ")
        } else {
            format!(" Clients ({shown}/{total}) [\"{}\" ] ", self.search_query)
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

        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(table_area);

        let filter_labels = &["All", "Wireless", "Wired", "VPN", "Guest"];
        let filter_line = sub_tabs::render_sub_tabs(filter_labels, self.filter_index());
        frame.render_widget(Paragraph::new(filter_line), layout[0]);

        let header = Row::new(vec![
            Cell::from("Type").style(theme::table_header()),
            Cell::from("Name").style(theme::table_header()),
            Cell::from("IP").style(theme::table_header()),
            Cell::from("MAC").style(theme::table_header()),
            Cell::from("Signal").style(theme::table_header()),
            Cell::from("TX/RX").style(theme::table_header()),
            Cell::from("Duration").style(theme::table_header()),
        ]);

        let selected_idx = if self.detail_open {
            self.detail_client_index(&filtered)
                .unwrap_or_else(|| self.selected_index())
        } else {
            self.selected_index()
        };

        let rows: Vec<Row> = filtered
            .iter()
            .enumerate()
            .map(|(index, client)| self.render_table_row(index, selected_idx, client))
            .collect();

        let widths = [
            Constraint::Length(3),
            Constraint::Min(14),
            Constraint::Length(15),
            Constraint::Length(17),
            Constraint::Length(6),
            Constraint::Length(11),
            Constraint::Length(8),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .row_highlight_style(theme::table_selected());

        let mut state = self.table_state;
        frame.render_stateful_widget(table, layout[1], &mut state);

        let hints = Line::from(vec![
            Span::styled("  j/k ", theme::key_hint_key()),
            Span::styled("navigate  ", theme::key_hint()),
            Span::styled("Enter ", theme::key_hint_key()),
            Span::styled("detail  ", theme::key_hint()),
            Span::styled("Tab ", theme::key_hint_key()),
            Span::styled("filter  ", theme::key_hint()),
            Span::styled("b ", theme::key_hint_key()),
            Span::styled("block  ", theme::key_hint()),
            Span::styled("x ", theme::key_hint_key()),
            Span::styled("kick", theme::key_hint()),
        ]);
        frame.render_widget(Paragraph::new(hints), layout[2]);

        if let Some(detail_area) = detail_area
            && let Some(client) = self.detail_client(&filtered)
        {
            self.render_detail(frame, detail_area, client);
        }
    }

    fn render_table_row(&self, index: usize, selected_idx: usize, client: &Client) -> Row<'static> {
        let is_selected = index == selected_idx;
        let prefix = if is_selected { "▸" } else { " " };

        let type_char = match client.client_type {
            ClientType::Wireless => "W",
            ClientType::Wired => "E",
            ClientType::Vpn => "V",
            ClientType::Teleport => "T",
            _ => "?",
        };
        let type_str = if client.is_guest {
            format!("{prefix}G")
        } else {
            format!("{prefix}{type_char}")
        };

        let name = client
            .name
            .as_deref()
            .or(client.hostname.as_deref())
            .unwrap_or("unknown");
        let ip = client
            .ip
            .map_or_else(|| "─".into(), |client_ip| client_ip.to_string());
        let mac = client.mac.to_string();
        let signal = client
            .wireless
            .as_ref()
            .and_then(|wireless| wireless.signal_dbm)
            .map_or("····", |dbm| {
                if dbm >= -50 {
                    "▂▄▆█"
                } else if dbm >= -60 {
                    "▂▄▆ "
                } else if dbm >= -70 {
                    "▂▄  "
                } else if dbm >= -80 {
                    "▂   "
                } else {
                    "·   "
                }
            });
        let traffic =
            bytes_fmt::fmt_tx_rx(client.tx_bytes.unwrap_or(0), client.rx_bytes.unwrap_or(0));
        let duration = client.connected_at.map_or_else(
            || "─".into(),
            |ts| {
                let dur = chrono::Utc::now().signed_duration_since(ts);
                #[allow(clippy::cast_sign_loss)]
                let secs = dur.num_seconds().max(0) as u64;
                bytes_fmt::fmt_uptime(secs)
            },
        );

        let type_color = if client.is_guest {
            theme::warning()
        } else {
            match client.client_type {
                ClientType::Wireless => theme::accent_secondary(),
                ClientType::Vpn => theme::accent_primary(),
                ClientType::Teleport => theme::accent_tertiary(),
                _ => theme::text_secondary(),
            }
        };

        let signal_color = client
            .wireless
            .as_ref()
            .and_then(|wireless| wireless.signal_dbm)
            .map_or(theme::border_unfocused(), |dbm| {
                if dbm >= -50 {
                    theme::success()
                } else if dbm >= -60 {
                    theme::accent_secondary()
                } else if dbm >= -70 {
                    theme::warning()
                } else if dbm >= -80 {
                    theme::accent_tertiary()
                } else {
                    theme::error()
                }
            });

        let row_style = if is_selected {
            theme::table_selected()
        } else {
            theme::table_row()
        };

        Row::new(vec![
            Cell::from(type_str).style(Style::default().fg(type_color)),
            Cell::from(name.to_string()).style(
                Style::default()
                    .fg(theme::accent_secondary())
                    .add_modifier(if is_selected {
                        Modifier::BOLD
                    } else {
                        Modifier::empty()
                    }),
            ),
            Cell::from(ip).style(Style::default().fg(theme::accent_tertiary())),
            Cell::from(mac),
            Cell::from(signal.to_string()).style(Style::default().fg(signal_color)),
            Cell::from(traffic),
            Cell::from(duration),
        ])
        .style(row_style)
    }

    #[allow(clippy::unused_self, clippy::too_many_lines, clippy::as_conversions)]
    fn render_detail(&self, frame: &mut Frame, area: Rect, client: &Client) {
        let name = client
            .name
            .as_deref()
            .or(client.hostname.as_deref())
            .unwrap_or("Unknown");
        let ip = client
            .ip
            .map_or_else(|| "─".into(), |client_ip| client_ip.to_string());
        let mac = client.mac.to_string();
        let type_str = format!("{:?}", client.client_type);

        let title = format!(" {name}  ·  {type_str}  ·  {ip}  ·  {mac} ");
        let block = Block::default()
            .title(title)
            .title_style(theme::title_style())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_focused());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let network = client
            .network_id
            .as_ref()
            .map_or_else(|| "─".into(), std::string::ToString::to_string);
        let signal = client
            .wireless
            .as_ref()
            .and_then(|wireless| wireless.signal_dbm)
            .map_or_else(|| "─".into(), |dbm| format!("{dbm} dBm"));
        let channel = client
            .wireless
            .as_ref()
            .and_then(|wireless| wireless.channel)
            .map_or_else(|| "─".into(), |channel| channel.to_string());
        let ssid = client
            .wireless
            .as_ref()
            .and_then(|wireless| wireless.ssid.as_deref())
            .unwrap_or("─");
        let tx = client
            .tx_bytes
            .map_or_else(|| "─".into(), bytes_fmt::fmt_bytes_short);
        let rx = client
            .rx_bytes
            .map_or_else(|| "─".into(), bytes_fmt::fmt_bytes_short);
        let duration = client.connected_at.map_or_else(
            || "─".into(),
            |ts| {
                let dur = chrono::Utc::now().signed_duration_since(ts);
                #[allow(clippy::cast_sign_loss)]
                let secs = dur.num_seconds().max(0) as u64;
                bytes_fmt::fmt_uptime(secs)
            },
        );
        let guest = if client.is_guest { "yes" } else { "no" };
        let blocked = if client.blocked { "yes" } else { "no" };

        let detail_layout =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(inner);

        let lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "  Network        ",
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(network, Style::default().fg(theme::accent_secondary())),
                Span::styled(
                    "       SSID         ",
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(ssid, Style::default().fg(theme::accent_secondary())),
            ]),
            Line::from(vec![
                Span::styled(
                    "  Signal         ",
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(&signal, Style::default().fg(theme::accent_secondary())),
                Span::styled(
                    "       Channel      ",
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(&channel, Style::default().fg(theme::accent_secondary())),
            ]),
            Line::from(vec![
                Span::styled(
                    "  TX             ",
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(&tx, Style::default().fg(theme::accent_tertiary())),
                Span::styled(
                    "       RX           ",
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(&rx, Style::default().fg(theme::accent_tertiary())),
            ]),
            Line::from(vec![
                Span::styled(
                    "  Duration       ",
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(&duration, Style::default().fg(theme::accent_secondary())),
            ]),
            Line::from(vec![
                Span::styled(
                    "  Guest          ",
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(guest, Style::default().fg(theme::text_secondary())),
                Span::styled(
                    "       Blocked      ",
                    Style::default().fg(theme::text_secondary()),
                ),
                Span::styled(
                    blocked,
                    Style::default().fg(if client.blocked {
                        theme::error()
                    } else {
                        theme::text_secondary()
                    }),
                ),
            ]),
        ];
        frame.render_widget(Paragraph::new(lines), detail_layout[0]);

        let hints = Line::from(vec![
            Span::styled("  b ", theme::key_hint_key()),
            Span::styled("block  ", theme::key_hint()),
            Span::styled("B ", theme::key_hint_key()),
            Span::styled("unblock  ", theme::key_hint()),
            Span::styled("x ", theme::key_hint_key()),
            Span::styled("kick  ", theme::key_hint()),
            Span::styled("Esc ", theme::key_hint_key()),
            Span::styled("back", theme::key_hint()),
        ]);
        frame.render_widget(Paragraph::new(hints), detail_layout[1]);
    }
}
