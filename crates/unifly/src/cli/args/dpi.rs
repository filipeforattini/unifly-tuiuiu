use clap::{Args, Subcommand};

use super::ListArgs;

#[derive(Debug, Args)]
pub struct DpiArgs {
    #[command(subcommand)]
    pub command: DpiCommand,
}

#[derive(Debug, Subcommand)]
pub enum DpiCommand {
    /// List DPI applications
    Apps(ListArgs),

    /// List DPI categories
    Categories(ListArgs),

    /// Show DPI status (legacy API)
    Status,

    /// Enable Deep Packet Inspection (legacy API)
    Enable,

    /// Disable Deep Packet Inspection (legacy API)
    Disable,
}
