use super::super::{SettingsField, SettingsScreen};

use crate::tui::forms::widgets::render_input_field;
use crate::tui::theme;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

impl SettingsScreen {
    pub(super) fn render_auth_selector(&self, frame: &mut Frame, area: Rect) {
        if area.height < 3 {
            return;
        }

        let active = self.active_field == SettingsField::AuthMode;
        let label_style = if active {
            Style::default().fg(theme::accent_secondary())
        } else {
            Style::default().fg(theme::text_secondary())
        };
        frame.render_widget(
            Paragraph::new(Span::styled("  Auth Mode", label_style)),
            Rect::new(area.x, area.y, area.width, 1),
        );

        let border_color = if active {
            theme::accent_primary()
        } else {
            theme::border_unfocused()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color));

        let block_area = Rect::new(area.x, area.y + 1, area.width, 3.min(area.height - 1));
        let inner = block.inner(block_area);
        frame.render_widget(block, block_area);

        let arrow_style = if active {
            Style::default().fg(theme::accent_primary())
        } else {
            Style::default().fg(theme::border_unfocused())
        };
        let value_style = if active {
            Style::default()
                .fg(theme::accent_secondary())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::text_secondary())
        };

        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(" \u{25C2} ", arrow_style),
                Span::styled(self.draft.auth_mode.label(), value_style),
                Span::styled(" \u{25B8}", arrow_style),
            ])),
            inner,
        );
    }

    pub(super) fn render_toggle(
        &self,
        frame: &mut Frame,
        area: Rect,
        label: &str,
        value: bool,
        active: bool,
    ) {
        if area.height < 1 {
            return;
        }

        let marker = if value { "[\u{2713}]" } else { "[ ]" };
        let marker_style = if active {
            Style::default().fg(theme::accent_primary())
        } else if value {
            Style::default().fg(theme::success())
        } else {
            Style::default().fg(theme::border_unfocused())
        };
        let label_style = if active {
            Style::default().fg(theme::accent_secondary())
        } else {
            Style::default().fg(theme::text_secondary())
        };

        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(format!("  {marker} "), marker_style),
                Span::styled(label, label_style),
            ])),
            area,
        );
    }

    pub(super) fn render_theme_field(&self, frame: &mut Frame, area: Rect) {
        if area.height < 1 {
            return;
        }

        let active = self.active_field == SettingsField::Theme;
        let label_style = if active {
            Style::default().fg(theme::accent_secondary())
        } else {
            Style::default().fg(theme::text_secondary())
        };
        let value_style = if active {
            Style::default()
                .fg(theme::accent_primary())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::text_primary())
        };
        let hint = if active { "  (Enter to change)" } else { "" };

        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled("  Theme: ", label_style),
                Span::styled(opaline::current().meta.name.clone(), value_style),
                Span::styled(hint, Style::default().fg(theme::border_unfocused())),
            ])),
            area,
        );
    }

    pub(super) fn render_editing(&self, frame: &mut Frame, area: Rect) {
        let field_layout = self.field_layout();
        let mut constraints: Vec<_> = field_layout
            .iter()
            .map(|(_, height)| Constraint::Length(*height))
            .collect();
        constraints.push(Constraint::Min(0));

        let fields_area = Rect::new(
            area.x + 1,
            area.y,
            area.width.saturating_sub(2),
            area.height,
        );
        let chunks = Layout::vertical(constraints).split(fields_area);

        for ((field, _), chunk) in field_layout.iter().zip(chunks.iter().copied()) {
            match field {
                SettingsField::Url => render_input_field(
                    frame,
                    chunk,
                    "  Controller URL",
                    &self.draft.url,
                    self.active_field == SettingsField::Url,
                    false,
                ),
                SettingsField::AuthMode => self.render_auth_selector(frame, chunk),
                SettingsField::ApiKey => render_input_field(
                    frame,
                    chunk,
                    "  API Key",
                    &self.draft.api_key,
                    self.active_field == SettingsField::ApiKey,
                    true,
                ),
                SettingsField::Username => render_input_field(
                    frame,
                    chunk,
                    "  Username",
                    &self.draft.username,
                    self.active_field == SettingsField::Username,
                    false,
                ),
                SettingsField::Password => render_input_field(
                    frame,
                    chunk,
                    "  Password",
                    &self.draft.password,
                    self.active_field == SettingsField::Password,
                    !self.show_password,
                ),
                SettingsField::Site => render_input_field(
                    frame,
                    chunk,
                    "  Site",
                    &self.draft.site,
                    self.active_field == SettingsField::Site,
                    false,
                ),
                SettingsField::Insecure => self.render_toggle(
                    frame,
                    chunk,
                    "Skip TLS verification (insecure)",
                    self.draft.insecure,
                    self.active_field == SettingsField::Insecure,
                ),
                SettingsField::Theme => self.render_theme_field(frame, chunk),
            }
        }
    }

    pub(super) fn render_testing(&self, frame: &mut Frame, area: Rect) {
        let layout = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(area);

        let throbber = throbber_widgets_tui::Throbber::default()
            .label("  Testing connection...")
            .style(Style::default().fg(theme::accent_secondary()))
            .throbber_style(Style::default().fg(theme::accent_primary()));

        frame.render_stateful_widget(throbber, layout[1], &mut self.throbber_state.clone());
        frame.render_widget(
            Paragraph::new(Span::styled(
                format!("  Connecting to {}", self.draft.url.trim()),
                Style::default().fg(theme::border_unfocused()),
            )),
            layout[2],
        );
    }
}
