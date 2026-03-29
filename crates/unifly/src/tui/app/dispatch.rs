use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use tokio_util::sync::CancellationToken;
use tracing::debug;

use unifly_api::{Command, Controller};

use super::{App, ConnectionStatus};
use crate::tui::action::{Action, ConfirmAction, Notification, StatsPeriod};
use crate::tui::component::Component;
use crate::tui::screen::ScreenId;

impl App {
    /// Map a key event to an action. Global keys are handled here;
    /// screen-specific keys are delegated to the active screen component.
    pub(super) fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if let Some(action) = self.handle_special_screen_key_event(key)? {
            return Ok(Some(action));
        }

        if self.active_screen == ScreenId::Setup || self.active_screen == ScreenId::Settings {
            return Ok(None);
        }

        if self.pending_confirm.is_some() {
            return Ok(match key.code {
                KeyCode::Char('y' | 'Y') => Some(Action::ConfirmYes),
                KeyCode::Char('n' | 'N') | KeyCode::Esc => Some(Action::ConfirmNo),
                _ => None,
            });
        }

        if self.search_active {
            return self.handle_search_key_event(key);
        }

        if self.help_visible {
            return Ok(match key.code {
                KeyCode::Esc | KeyCode::Char('?') => Some(Action::ToggleHelp),
                _ => None,
            });
        }

        if let Some(action) = self.handle_global_key_event(key) {
            return Ok(Some(action));
        }

        self.forward_key_to_active_screen(key)
    }

    /// Handle mouse events (delegate to active screen).
    pub(super) fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Result<Option<Action>> {
        if let Some(screen) = self.screens.get_mut(&self.active_screen) {
            return screen.handle_mouse_event(mouse);
        }
        Ok(None)
    }

    /// Process a single action — update app state and propagate to components.
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    pub(super) fn process_action(&mut self, action: &Action) -> Result<()> {
        match action {
            Action::Quit => {
                self.running = false;
            }
            Action::Resize(w, h) => {
                self.terminal_size = (*w, *h);
            }
            Action::SwitchScreen(target) => self.switch_screen(*target)?,
            Action::GoBack => {
                if let Some(prev) = self.previous_screen.take() {
                    self.action_tx.send(Action::SwitchScreen(prev))?;
                }
            }
            Action::ToggleHelp => {
                self.help_visible = !self.help_visible;
            }
            Action::OpenSearch => {
                self.search_active = true;
                self.search_query.clear();
            }
            Action::CloseSearch => {
                self.search_active = false;
                self.search_query.clear();
            }
            Action::Connected => {
                self.connection_status = ConnectionStatus::Connected;
            }
            Action::Disconnected(_) => {
                self.connection_status = ConnectionStatus::Disconnected;
            }
            Action::Reconnecting => {
                self.connection_status = ConnectionStatus::Reconnecting;
            }
            Action::Render => {}
            Action::Tick => {
                self.forward_to_all_screens(action)?;

                if let Some((_, created)) = &self.notification
                    && created.elapsed() > Duration::from_secs(3)
                {
                    self.notification = None;
                }

                if self.active_screen == ScreenId::Stats
                    && let Some(last) = self.last_stats_fetch
                    && last.elapsed() > Duration::from_secs(60)
                {
                    let _ = self.action_tx.send(Action::RequestStats(self.stats_period));
                }
            }
            Action::DevicesUpdated(_)
            | Action::ClientsUpdated(_)
            | Action::NetworksUpdated(_)
            | Action::FirewallPoliciesUpdated(_)
            | Action::FirewallZonesUpdated(_)
            | Action::AclRulesUpdated(_)
            | Action::WifiBroadcastsUpdated(_)
            | Action::EventReceived(_)
            | Action::HealthUpdated(_)
            | Action::SiteUpdated(_)
            | Action::StatsUpdated(_)
            | Action::NetworkEditResult(_) => {
                self.forward_to_all_screens(action)?;
            }
            Action::RequestRestart(id) => {
                let name = self.resolve_device_name(id);
                self.action_tx
                    .send(Action::ShowConfirm(ConfirmAction::RestartDevice {
                        id: id.clone(),
                        name,
                    }))?;
            }
            Action::RequestUnadopt(id) => {
                let name = self.resolve_device_name(id);
                self.action_tx
                    .send(Action::ShowConfirm(ConfirmAction::UnadoptDevice {
                        id: id.clone(),
                        name,
                    }))?;
            }
            Action::RequestLocate(id) => {
                if let Some(mac) = self.resolve_device_mac(id) {
                    self.execute_command(
                        Command::LocateDevice {
                            mac: mac.clone(),
                            enable: true,
                        },
                        format!("Locating {mac}"),
                    );
                }
            }
            Action::RequestBlockClient(id) => {
                let name = self.resolve_client_name(id);
                self.action_tx
                    .send(Action::ShowConfirm(ConfirmAction::BlockClient {
                        id: id.clone(),
                        name,
                    }))?;
            }
            Action::RequestUnblockClient(id) => {
                let name = self.resolve_client_name(id);
                self.action_tx
                    .send(Action::ShowConfirm(ConfirmAction::UnblockClient {
                        id: id.clone(),
                        name,
                    }))?;
            }
            Action::RequestForgetClient(id) => {
                let name = self.resolve_client_name(id);
                self.action_tx
                    .send(Action::ShowConfirm(ConfirmAction::ForgetClient {
                        id: id.clone(),
                        name,
                    }))?;
            }
            Action::RequestKickClient(id) => {
                if let Some(mac) = self.resolve_client_mac(id) {
                    let name = self.resolve_client_name(id);
                    self.execute_command(Command::KickClient { mac }, format!("Kicked {name}"));
                }
            }
            Action::ShowConfirm(confirm) => {
                self.pending_confirm = Some(confirm.clone());
            }
            Action::ConfirmYes => {
                if let Some(confirm) = self.pending_confirm.take() {
                    self.execute_confirm(confirm);
                }
            }
            Action::ConfirmNo => {
                self.pending_confirm = None;
            }
            Action::NetworkSave(id, update) => {
                self.execute_command(
                    Command::UpdateNetwork {
                        id: id.clone(),
                        update: *update.clone(),
                    },
                    "Updated network".into(),
                );
            }
            Action::RequestStats(period) => {
                self.stats_period = *period;
                self.last_stats_fetch = Some(std::time::Instant::now());
                self.fetch_stats(*period);
            }
            Action::OnboardingComplete { config, .. } => {
                self.handle_onboarding_complete(config)?;
            }
            Action::OnboardingTestResult(_) => {
                self.forward_to_screen(ScreenId::Setup, action)?;
            }
            Action::OpenSettings => {
                self.open_settings()?;
            }
            Action::CloseSettings => {
                self.close_settings();
            }
            Action::SettingsTestResult(_) => {
                self.forward_to_screen(ScreenId::Settings, action)?;
            }
            Action::SettingsApply { config, .. } => {
                self.apply_settings(config)?;
            }
            Action::Notify(notification) => {
                self.show_notification(notification.clone());
            }
            Action::DismissNotification => {
                self.notification = None;
            }
            other => {
                self.forward_to_screen(self.active_screen, other)?;
            }
        }

        Ok(())
    }

    fn handle_special_screen_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if self.active_screen == ScreenId::Setup || self.active_screen == ScreenId::Settings {
            if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                return Ok(Some(Action::Quit));
            }

            if let Some(screen) = self.screens.get_mut(&self.active_screen) {
                return screen.handle_key_event(key);
            }
        }

        Ok(None)
    }

    fn handle_search_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        Ok(match key.code {
            KeyCode::Esc => {
                self.search_query.clear();
                Some(Action::CloseSearch)
            }
            KeyCode::Enter => Some(Action::SearchSubmit),
            KeyCode::Backspace => {
                self.search_query.pop();
                Some(Action::SearchInput(self.search_query.clone()))
            }
            KeyCode::Char(c) => {
                self.search_query.push(c);
                Some(Action::SearchInput(self.search_query.clone()))
            }
            _ => None,
        })
    }

    fn handle_global_key_event(&self, key: KeyEvent) -> Option<Action> {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c'))
            | (KeyModifiers::NONE, KeyCode::Char('q')) => Some(Action::Quit),
            (KeyModifiers::NONE, KeyCode::Char('?')) => Some(Action::ToggleHelp),
            (KeyModifiers::NONE, KeyCode::Char('/')) => Some(Action::OpenSearch),
            (KeyModifiers::NONE, KeyCode::Char(',')) => Some(Action::OpenSettings),
            (KeyModifiers::NONE, KeyCode::Char(c @ '1'..='8')) => {
                #[allow(clippy::cast_possible_truncation, clippy::as_conversions)]
                let n = c.to_digit(10).unwrap_or(0) as u8;
                ScreenId::from_number(n).map(Action::SwitchScreen)
            }
            (KeyModifiers::NONE, KeyCode::Tab) => {
                Some(Action::SwitchScreen(self.active_screen.next()))
            }
            (KeyModifiers::SHIFT, KeyCode::BackTab) => {
                Some(Action::SwitchScreen(self.active_screen.prev()))
            }
            (KeyModifiers::NONE, KeyCode::Esc) => Some(Action::GoBack),
            _ => None,
        }
    }

    fn forward_key_to_active_screen(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if let Some(screen) = self.screens.get_mut(&self.active_screen) {
            return screen.handle_key_event(key);
        }

        Ok(None)
    }

    fn switch_screen(&mut self, target: ScreenId) -> Result<()> {
        if target == self.active_screen {
            return Ok(());
        }

        debug!("switching screen: {} → {}", self.active_screen, target);

        if let Some(screen) = self.screens.get_mut(&self.active_screen) {
            screen.set_focused(false);
        }

        self.previous_screen = Some(self.active_screen);
        self.active_screen = target;

        if let Some(screen) = self.screens.get_mut(&self.active_screen) {
            screen.set_focused(true);
        }

        if target == ScreenId::Stats {
            self.action_tx
                .send(Action::RequestStats(StatsPeriod::default()))?;
        }

        Ok(())
    }

    fn forward_to_all_screens(&mut self, action: &Action) -> Result<()> {
        for screen in self.screens.values_mut() {
            if let Some(follow_up) = screen.update(action)? {
                self.action_tx.send(follow_up)?;
            }
        }

        Ok(())
    }

    fn forward_to_screen(&mut self, screen_id: ScreenId, action: &Action) -> Result<()> {
        if let Some(screen) = self.screens.get_mut(&screen_id)
            && let Some(follow_up) = screen.update(action)?
        {
            self.action_tx.send(follow_up)?;
        }

        Ok(())
    }

    fn handle_onboarding_complete(&mut self, config: &unifly_api::ControllerConfig) -> Result<()> {
        self.screens.remove(&ScreenId::Setup);

        let controller = Controller::new(config.clone());
        self.controller = Some(controller.clone());
        self.active_screen = ScreenId::Dashboard;

        if let Some(screen) = self.screens.get_mut(&ScreenId::Dashboard) {
            screen.set_focused(true);
        }

        let cancel = self.data_cancel.clone();
        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            crate::tui::data_bridge::spawn_data_bridge(controller, tx, cancel).await;
        });

        self.action_tx
            .send(Action::Notify(Notification::success("Connected!")))?;

        Ok(())
    }

    fn open_settings(&mut self) -> Result<()> {
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

    fn close_settings(&mut self) {
        self.screens.remove(&ScreenId::Settings);
        let target = self.previous_screen.take().unwrap_or(ScreenId::Dashboard);
        self.active_screen = target;

        if let Some(screen) = self.screens.get_mut(&target) {
            screen.set_focused(true);
        }
    }

    fn apply_settings(&mut self, config: &unifly_api::ControllerConfig) -> Result<()> {
        self.data_cancel.cancel();
        self.data_cancel = CancellationToken::new();

        let controller = Controller::new(config.clone());
        self.controller = Some(controller.clone());

        let cancel = self.data_cancel.clone();
        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            crate::tui::data_bridge::spawn_data_bridge(controller, tx, cancel).await;
        });

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
}
