use super::{AuthMode, ControllerProfileDraft, SettingsField, SettingsScreen, SettingsState};

impl SettingsField {
    pub(super) const ALL: [SettingsField; 9] = [
        Self::Url,
        Self::AuthMode,
        Self::ApiKey,
        Self::Username,
        Self::Password,
        Self::Site,
        Self::Insecure,
        Self::Theme,
        Self::ShowDonate,
    ];

    pub(super) fn visible_for(self, mode: AuthMode) -> bool {
        match self {
            Self::Url
            | Self::AuthMode
            | Self::Site
            | Self::Insecure
            | Self::Theme
            | Self::ShowDonate => true,
            Self::ApiKey => matches!(mode, AuthMode::ApiKey | AuthMode::Hybrid),
            Self::Username | Self::Password => {
                matches!(mode, AuthMode::Legacy | AuthMode::Hybrid)
            }
        }
    }
}

impl SettingsScreen {
    pub fn new() -> Self {
        let mut screen = Self {
            focused: false,
            action_tx: None,
            state: SettingsState::Editing,
            active_field: SettingsField::Url,
            draft: ControllerProfileDraft::default(),
            auth_mode_index: 0,
            show_password: false,
            profile_name: "default".into(),
            test_error: None,
            throbber_state: throbber_widgets_tui::ThrobberState::default(),
            last_area: std::cell::Cell::new(ratatui::layout::Rect::default()),
            theme_selector: std::cell::RefCell::new(None),
            show_donate: true,
        };
        screen.load_from_config();
        screen
    }

    fn load_from_config(&mut self) {
        let Ok(cfg) = crate::config::load_config() else {
            return;
        };

        self.show_donate = cfg.defaults.show_donate;

        let profile_name = cfg.default_profile.as_deref().unwrap_or("default");
        let Some(profile) = cfg.profiles.get(profile_name) else {
            return;
        };

        self.profile_name = profile_name.to_string();
        self.draft = ControllerProfileDraft::from_profile(profile);
        self.auth_mode_index = AuthMode::ALL
            .iter()
            .position(|&mode| mode == self.draft.auth_mode)
            .unwrap_or(0);
    }

    pub(super) fn visible_fields(&self) -> Vec<SettingsField> {
        SettingsField::ALL
            .iter()
            .copied()
            .filter(|field| field.visible_for(self.draft.auth_mode))
            .collect()
    }

    pub(super) fn field_layout(&self) -> Vec<(SettingsField, u16)> {
        self.visible_fields()
            .into_iter()
            .map(|field| {
                let height = match field {
                    SettingsField::Insecure | SettingsField::ShowDonate => 1,
                    SettingsField::Theme => 2,
                    _ => 4,
                };
                (field, height)
            })
            .collect()
    }

    pub(super) fn focus_next(&mut self) {
        let fields = self.visible_fields();
        let pos = fields
            .iter()
            .position(|&field| field == self.active_field)
            .unwrap_or(0);
        self.active_field = fields[(pos + 1) % fields.len()];
    }

    pub(super) fn focus_prev(&mut self) {
        let fields = self.visible_fields();
        let pos = fields
            .iter()
            .position(|&field| field == self.active_field)
            .unwrap_or(0);
        self.active_field = fields[(pos + fields.len() - 1) % fields.len()];
    }

    pub(super) fn clamp_focus(&mut self) {
        if !self.active_field.visible_for(self.draft.auth_mode) {
            self.active_field = SettingsField::AuthMode;
        }
    }

    pub(super) fn cycle_auth_mode_previous(&mut self) {
        if self.auth_mode_index == 0 {
            self.auth_mode_index = AuthMode::ALL.len() - 1;
        } else {
            self.auth_mode_index -= 1;
        }
        self.draft.auth_mode = AuthMode::ALL[self.auth_mode_index];
        self.clamp_focus();
    }

    pub(super) fn cycle_auth_mode_next(&mut self) {
        if self.auth_mode_index < AuthMode::ALL.len() - 1 {
            self.auth_mode_index += 1;
        } else {
            self.auth_mode_index = 0;
        }
        self.draft.auth_mode = AuthMode::ALL[self.auth_mode_index];
        self.clamp_focus();
    }

    pub(super) fn active_input_mut(&mut self) -> Option<&mut String> {
        match self.active_field {
            SettingsField::Url => Some(&mut self.draft.url),
            SettingsField::ApiKey => Some(&mut self.draft.api_key),
            SettingsField::Username => Some(&mut self.draft.username),
            SettingsField::Password => Some(&mut self.draft.password),
            SettingsField::Site => Some(&mut self.draft.site),
            SettingsField::AuthMode
            | SettingsField::Insecure
            | SettingsField::Theme
            | SettingsField::ShowDonate => None,
        }
    }

    pub(super) fn toggle_show_donate(&mut self) {
        self.show_donate = !self.show_donate;

        if let Ok(mut cfg) = crate::config::load_config() {
            cfg.defaults.show_donate = self.show_donate;
            let _ = crate::config::save_config(&cfg);
        }

        if let Some(ref tx) = self.action_tx {
            let _ = tx.send(crate::tui::action::Action::SetShowDonate(self.show_donate));
        }
    }

    pub(super) fn save_theme_preference(theme_id: &str) {
        if let Ok(mut cfg) = crate::config::load_config() {
            cfg.defaults.theme = Some(theme_id.to_string());
            let _ = crate::config::save_config(&cfg);
        }
    }

    pub(super) fn validate(&self) -> std::result::Result<(), String> {
        self.draft.validate_complete()
    }

    pub(super) fn submit_connection_test(&mut self) {
        if let Err(message) = self.validate() {
            self.test_error = Some(message);
        } else {
            self.start_connection_test();
        }
    }

    fn build_profile(&self) -> crate::config::Profile {
        self.draft.to_profile()
    }

    fn start_connection_test(&mut self) {
        self.state = SettingsState::Testing;
        self.test_error = None;

        let profile = self.build_profile();
        let profile_name = self.profile_name.clone();

        let Some(tx) = self.action_tx.clone() else {
            return;
        };

        tokio::spawn(async move {
            let result = match crate::config::profile_to_controller_config(&profile, &profile_name)
            {
                Ok(config) => {
                    let controller = unifly_api::Controller::new(config);
                    match controller.connect().await {
                        Ok(()) => {
                            controller.disconnect().await;
                            let mut cfg = crate::config::load_config().unwrap_or_default();
                            cfg.profiles.insert(profile_name.clone(), profile);
                            if cfg.default_profile.is_none() {
                                cfg.default_profile = Some(profile_name.clone());
                            }
                            if let Err(error) = crate::config::save_config(&cfg) {
                                Err(format!("Connected, but failed to save config: {error}"))
                            } else {
                                Ok(())
                            }
                        }
                        Err(error) => Err(format!("{error}")),
                    }
                }
                Err(error) => Err(format!("{error}")),
            };

            let _ = tx.send(crate::tui::action::Action::SettingsTestResult(result));
        });
    }

    pub(super) fn send_apply(&self) {
        let profile = self.build_profile();
        let Some(tx) = self.action_tx.clone() else {
            return;
        };

        match crate::config::profile_to_controller_config(&profile, &self.profile_name) {
            Ok(config) => {
                let _ = tx.send(crate::tui::action::Action::SettingsApply {
                    profile_name: self.profile_name.clone(),
                    config: Box::new(config),
                });
            }
            Err(error) => {
                let _ = tx.send(crate::tui::action::Action::Notify(
                    crate::tui::action::Notification::error(format!("{error}")),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AuthMode, SettingsField, SettingsScreen};

    fn test_screen() -> SettingsScreen {
        let mut screen = SettingsScreen::new();
        screen.draft.url = "https://console.example.com".into();
        screen.draft.api_key = "api-key".into();
        screen.draft.username = "bliss".into();
        screen.draft.password = "hunter2".into();
        screen.draft.site = "default".into();
        screen.draft.auth_mode = AuthMode::ApiKey;
        screen.auth_mode_index = 0;
        screen.active_field = SettingsField::Url;
        screen.test_error = None;
        screen
    }

    #[test]
    fn field_layout_hides_unused_credentials() {
        let mut screen = test_screen();
        screen.draft.auth_mode = AuthMode::Legacy;
        screen.auth_mode_index = 1;

        let fields: Vec<_> = screen
            .field_layout()
            .into_iter()
            .map(|(field, _)| field)
            .collect();

        assert!(!fields.contains(&SettingsField::ApiKey));
        assert!(fields.contains(&SettingsField::Username));
        assert!(fields.contains(&SettingsField::Password));
    }

    #[test]
    fn build_profile_omits_non_selected_auth_fields() {
        let mut screen = test_screen();
        screen.draft.auth_mode = AuthMode::ApiKey;

        let profile = screen.build_profile();

        assert_eq!(profile.api_key.as_deref(), Some("api-key"));
        assert_eq!(profile.username, None);
        assert_eq!(profile.password, None);
    }

    #[test]
    fn validate_requires_legacy_credentials() {
        let mut screen = test_screen();
        screen.draft.auth_mode = AuthMode::Legacy;
        screen.draft.username.clear();

        assert_eq!(screen.validate(), Err("Username cannot be empty".into()));
    }
}
