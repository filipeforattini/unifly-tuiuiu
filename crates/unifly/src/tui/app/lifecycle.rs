use color_eyre::eyre::Result;
use tokio_util::sync::CancellationToken;

use unifly_api::Controller;

use super::App;
use crate::tui::action::{Action, Notification};
use crate::tui::component::Component;
use crate::tui::screen::ScreenId;

impl App {
    pub(super) fn handle_onboarding_complete(
        &mut self,
        config: &unifly_api::ControllerConfig,
    ) -> Result<()> {
        self.screens.remove(&ScreenId::Setup);

        let controller = Controller::new(config.clone());
        self.controller = Some(controller.clone());
        self.active_screen = ScreenId::Dashboard;

        if let Some(screen) = self.screens.get_mut(&ScreenId::Dashboard) {
            screen.set_focused(true);
        }

        self.spawn_data_bridge(controller);
        self.action_tx
            .send(Action::Notify(Notification::success("Connected!")))?;

        Ok(())
    }

    pub(super) fn open_settings(&mut self) -> Result<()> {
        if self.active_screen == ScreenId::Settings || self.active_screen == ScreenId::Setup {
            return Ok(());
        }

        let mut screen = crate::tui::screens::settings::SettingsScreen::new();
        screen.init(self.action_tx.clone())?;
        self.screens.insert(ScreenId::Settings, Box::new(screen));
        self.previous_screen = Some(self.active_screen);

        if let Some(current) = self.screens.get_mut(&self.active_screen) {
            current.set_focused(false);
        }

        self.active_screen = ScreenId::Settings;

        if let Some(settings) = self.screens.get_mut(&ScreenId::Settings) {
            settings.set_focused(true);
        }

        Ok(())
    }

    pub(super) fn close_settings(&mut self) {
        self.screens.remove(&ScreenId::Settings);
        let target = self.previous_screen.take().unwrap_or(ScreenId::Dashboard);
        self.active_screen = target;

        if let Some(screen) = self.screens.get_mut(&target) {
            screen.set_focused(true);
        }
    }

    pub(super) fn apply_settings(&mut self, config: &unifly_api::ControllerConfig) -> Result<()> {
        self.reset_data_bridge();

        let controller = Controller::new(config.clone());
        self.controller = Some(controller.clone());
        self.spawn_data_bridge(controller);

        self.screens.remove(&ScreenId::Settings);
        self.active_screen = ScreenId::Dashboard;

        if let Some(screen) = self.screens.get_mut(&ScreenId::Dashboard) {
            screen.set_focused(true);
        }

        self.action_tx.send(Action::Notify(Notification::success(
            "Settings saved, reconnecting\u{2026}",
        )))?;

        Ok(())
    }

    fn spawn_data_bridge(&self, controller: Controller) {
        let cancel = self.data_cancel.clone();
        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            crate::tui::data_bridge::spawn_data_bridge(controller, tx, cancel).await;
        });
    }

    fn reset_data_bridge(&mut self) {
        self.data_cancel.cancel();
        self.data_cancel = CancellationToken::new();
    }
}
