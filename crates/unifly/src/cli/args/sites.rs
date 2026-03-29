use clap::{Args, Subcommand};

use super::ListArgs;

#[derive(Debug, Args)]
pub struct SitesArgs {
    #[command(subcommand)]
    pub command: SitesCommand,
}

#[derive(Debug, Subcommand)]
pub enum SitesCommand {
    /// List local sites
    #[command(alias = "ls")]
    List(ListArgs),

    /// Create a new site (legacy API)
    Create {
        /// Site name (internal reference)
        #[arg(long, required = true)]
        name: String,

        /// Site description (display name)
        #[arg(long, required = true)]
        description: String,
    },

    /// Delete a site (legacy API)
    Delete {
        /// Site name
        name: String,
    },
}
