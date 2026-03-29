mod chrome;
mod form;

use super::{SettingsScreen, SettingsState};

use crate::tui::theme;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::{Block, Clear, Paragraph};

pub(super) fn panel_rect(area: Rect) -> Rect {
    chrome::panel_rect(area)
}

impl SettingsScreen {
    pub(super) fn render_screen(&self, frame: &mut Frame, area: Rect) {
        self.last_area.set(area);

        frame.render_widget(
            Block::default().style(Style::default().bg(theme::bg_base())),
            area,
        );

        let inner = self.render_centered_panel(frame, area);
        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);

        if let Some(ref error) = self.test_error {
            frame.render_widget(
                Paragraph::new(Span::styled(error, Style::default().fg(theme::error())))
                    .alignment(Alignment::Center),
                layout[2],
            );
        }

        self.render_key_hints(frame, layout[3]);

        match self.state {
            SettingsState::Editing => self.render_editing(frame, layout[1]),
            SettingsState::Testing => self.render_testing(frame, layout[1]),
        }

        let mut selector = self.theme_selector.borrow_mut();
        if let Some(ref mut theme_selector) = *selector {
            let overlay = chrome::centered_rect(area, 80, 28);
            frame.render_widget(Clear, overlay);
            frame.render_stateful_widget(opaline::ThemeSelector::new(), overlay, theme_selector);
        }
    }
}
