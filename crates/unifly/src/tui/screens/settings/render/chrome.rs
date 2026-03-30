use super::super::{SettingsField, SettingsScreen, SettingsState};

use crate::tui::theme;
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

pub(super) fn panel_rect(area: Rect) -> Rect {
    let width = 62u16.min(area.width.saturating_sub(4));
    let height = 32u16.min(area.height.saturating_sub(2));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    Rect::new(area.x + x, area.y + y, width, height)
}

pub(super) fn centered_rect(area: Rect, cols: u16, rows: u16) -> Rect {
    let width = cols.min(area.width.saturating_sub(2));
    let height = rows.min(area.height.saturating_sub(2));
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}

impl SettingsScreen {
    #[allow(clippy::unused_self)]
    pub(super) fn render_centered_panel(&self, frame: &mut Frame, area: Rect) -> Rect {
        let panel = panel_rect(area);

        frame.render_widget(
            Block::default().style(Style::default().bg(theme::bg_base())),
            panel,
        );

        let block = Block::default()
            .title(Line::from(vec![
                Span::raw(" "),
                Span::styled(
                    "Settings",
                    Style::default()
                        .fg(theme::accent_secondary())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
            ]))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::accent_primary()));

        let inner = block.inner(panel);
        frame.render_widget(block, panel);
        inner
    }

    pub(super) fn render_key_hints(&self, frame: &mut Frame, area: Rect) {
        let hints = match self.state {
            SettingsState::Editing => {
                if self.active_field == SettingsField::AuthMode {
                    "\u{25C2}/\u{25B8} select  Tab next  Enter test & save  Esc cancel"
                } else if self.active_field == SettingsField::Insecure {
                    "Space toggle  Tab next  Enter test & save  Esc cancel"
                } else if self.active_field == SettingsField::Password {
                    "Ctrl+U reveal  Tab next  Enter test & save  Esc cancel"
                } else if self.active_field == SettingsField::Theme {
                    "Enter choose theme  Tab next  Esc cancel"
                } else {
                    "Tab next  Shift+Tab prev  Enter test & save  Esc cancel"
                }
            }
            SettingsState::Testing => "Esc cancel",
        };

        frame.render_widget(
            Paragraph::new(Span::styled(hints, theme::key_hint())).alignment(Alignment::Center),
            area,
        );
    }
}
