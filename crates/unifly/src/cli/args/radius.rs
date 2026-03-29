use clap::{Args, Subcommand};

use super::ListArgs;

#[derive(Debug, Args)]
pub struct RadiusArgs {
    #[command(subcommand)]
    pub command: RadiusCommand,
}

#[derive(Debug, Subcommand)]
pub enum RadiusCommand {
    /// List RADIUS profiles
    Profiles(ListArgs),
}
