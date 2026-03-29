use clap::{Args, Subcommand};

use super::ListArgs;

#[derive(Debug, Args)]
pub struct WansArgs {
    #[command(subcommand)]
    pub command: WansCommand,
}

#[derive(Debug, Subcommand)]
pub enum WansCommand {
    /// List WAN interfaces
    #[command(alias = "ls")]
    List(ListArgs),
}
