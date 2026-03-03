//! `unifly-tui` — Real-time terminal dashboard for UniFi network monitoring.

use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Result;
use secrecy::SecretString;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use unifly_api::{AuthCredentials, Controller, ControllerConfig, TlsVerification};

use unifly::config;
use unifly::tui::app::App;
use unifly::tui::terminal;
use unifly::tui::theme;

/// Terminal dashboard for monitoring and managing UniFi networks.
#[derive(Parser, Debug)]
#[command(name = "unifly-tui", version, about)]
struct Cli {
    /// UniFi Controller URL (e.g., https://192.168.1.1)
    #[arg(short = 'u', long, env = "UNIFI_URL")]
    url: Option<String>,

    /// Site name (defaults to "default")
    #[arg(short = 's', long, default_value = "default", env = "UNIFI_SITE")]
    site: String,

    /// API key for the Integration API
    #[arg(short = 'a', long, env = "UNIFI_API_KEY")]
    api_key: Option<String>,

    /// Config profile to use (defaults to the default_profile in config)
    #[arg(short = 'p', long, env = "UNIFI_PROFILE")]
    profile: Option<String>,

    /// Theme name (e.g., nord, dracula, silkcircuit-neon)
    #[arg(long, env = "UNIFLY_THEME")]
    theme: Option<String>,

    /// Accept self-signed TLS certificates
    #[arg(short = 'k', long, env = "UNIFI_INSECURE")]
    insecure: bool,

    /// Log file path (defaults to /tmp/unifly-tui.log)
    #[arg(long, default_value = "/tmp/unifly-tui.log")]
    log_file: PathBuf,

    /// Increase log verbosity (-v info, -vv debug, -vvv trace)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn setup_tracing(cli: &Cli) -> WorkerGuard {
    let log_level = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("unifly={log_level},unifly_api={log_level}")));

    let log_dir = cli
        .log_file
        .parent()
        .unwrap_or(std::path::Path::new("/tmp"));
    let log_filename = cli
        .log_file
        .file_name()
        .unwrap_or(std::ffi::OsStr::new("unifly-tui.log"));

    let file_appender = tracing_appender::rolling::never(log_dir, log_filename);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_target(true)
                .with_thread_ids(true),
        )
        .init();

    guard
}

fn build_controller(cli: &Cli) -> Option<Controller> {
    let url_str = cli.url.as_deref()?;
    let url = url_str.parse().expect("invalid controller URL");

    let api_key = SecretString::from(cli.api_key.as_ref()?.clone());

    let auth = try_hybrid_from_config(&api_key).unwrap_or(AuthCredentials::ApiKey(api_key));

    let tls = if cli.insecure {
        TlsVerification::DangerAcceptInvalid
    } else {
        TlsVerification::SystemDefaults
    };

    let controller_config = ControllerConfig {
        url,
        auth,
        site: cli.site.clone(),
        tls,
        timeout: std::time::Duration::from_secs(30),
        refresh_interval_secs: 10,
        websocket_enabled: true,
        polling_interval_secs: 10,
    };

    Some(Controller::new(controller_config))
}

fn try_hybrid_from_config(api_key: &SecretString) -> Option<AuthCredentials> {
    let cfg = config::load_config().ok()?;
    let name = cfg.default_profile.as_deref().unwrap_or("default");
    let profile = cfg.profiles.get(name)?;

    if profile.auth_mode != "hybrid" {
        return None;
    }

    let (username, password) = config::resolve_legacy_credentials(profile, name).ok()?;

    Some(AuthCredentials::Hybrid {
        api_key: api_key.clone(),
        username,
        password,
    })
}

fn build_controller_from_config(profile_name: Option<&str>) -> Option<Controller> {
    let cfg = match config::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            tracing::warn!("failed to load config: {e}");
            return None;
        }
    };

    let profile_name = profile_name
        .or(cfg.default_profile.as_deref())
        .unwrap_or("default");

    let Some(profile) = cfg.profiles.get(profile_name) else {
        tracing::warn!(
            "profile '{profile_name}' not found in config (available: {:?})",
            cfg.profiles.keys().collect::<Vec<_>>()
        );
        return None;
    };

    match config::profile_to_controller_config(profile, profile_name) {
        Ok(controller_config) => Some(Controller::new(controller_config)),
        Err(e) => {
            tracing::warn!("failed to build controller from profile '{profile_name}': {e}");
            None
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    terminal::install_hooks()?;

    let _log_guard = setup_tracing(&cli);

    let config_theme = config::load_config().ok().and_then(|c| c.defaults.theme);
    let theme_name = cli.theme.as_deref().or(config_theme.as_deref());
    theme::initialize(theme_name);

    info!(
        url = cli.url.as_deref().unwrap_or("(not set)"),
        site = %cli.site,
        "starting unifly-tui"
    );

    let controller =
        build_controller(&cli).or_else(|| build_controller_from_config(cli.profile.as_deref()));
    let mut app = App::new(controller);
    app.run().await?;

    Ok(())
}
