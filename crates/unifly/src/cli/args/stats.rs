use clap::{Args, Subcommand, ValueEnum};

#[derive(Debug, Args)]
pub struct StatsArgs {
    #[command(subcommand)]
    pub command: StatsCommand,
}

#[derive(Debug, Subcommand)]
pub enum StatsCommand {
    /// Site-level statistics (legacy API)
    Site(StatsQuery),

    /// Per-device statistics (legacy API)
    Device(StatsQuery),

    /// Per-client statistics (legacy API)
    Client(StatsQuery),

    /// Gateway statistics (legacy API)
    Gateway(StatsQuery),

    /// DPI traffic analysis (legacy API)
    Dpi {
        /// Analysis type: by-app or by-cat
        #[arg(long, default_value = "by-app", value_enum)]
        group_by: DpiGroupBy,

        /// Filter by MAC addresses (comma-separated)
        #[arg(long, value_delimiter = ',')]
        macs: Option<Vec<String>>,
    },
}

#[derive(Debug, Args)]
pub struct StatsQuery {
    /// Reporting interval
    #[arg(long, default_value = "hourly", value_enum)]
    pub interval: StatsInterval,

    /// Start time (ISO 8601 or Unix timestamp)
    #[arg(long)]
    pub start: Option<String>,

    /// End time (ISO 8601 or Unix timestamp)
    #[arg(long)]
    pub end: Option<String>,

    /// Attributes to include (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub attrs: Option<Vec<String>>,

    /// Filter by MAC addresses (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub macs: Option<Vec<String>>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum StatsInterval {
    #[value(name = "5m")]
    FiveMinutes,
    Hourly,
    Daily,
    Monthly,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum DpiGroupBy {
    ByApp,
    ByCat,
}
