use crossterm::event::{KeyCode, KeyEvent};

use super::StatsScreen;
use crate::tui::action::{Action, StatsPeriod};

impl StatsScreen {
    pub(super) fn handle_key_input(&mut self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('h') => Some(self.set_period(StatsPeriod::OneHour)),
            KeyCode::Char('d') => Some(self.set_period(StatsPeriod::TwentyFourHours)),
            KeyCode::Char('w') => Some(self.set_period(StatsPeriod::SevenDays)),
            KeyCode::Char('m') => Some(self.set_period(StatsPeriod::ThirtyDays)),
            KeyCode::Char('r') => Some(Action::RequestStats(self.period)),
            _ => None,
        }
    }

    fn set_period(&mut self, period: StatsPeriod) -> Action {
        self.period = period;
        Action::RequestStats(period)
    }
}
