use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct AdminArgs {
    #[command(subcommand)]
    pub command: AdminCommand,
}

#[derive(Debug, Subcommand)]
pub enum AdminCommand {
    /// List site administrators (legacy API)
    #[command(alias = "ls")]
    List,

    /// Invite a new administrator (legacy API)
    Invite {
        /// Admin name
        #[arg(long, required = true)]
        name: String,

        /// Admin email
        #[arg(long, required = true)]
        email: String,

        /// Role: admin or readonly
        #[arg(long, default_value = "admin")]
        role: String,
    },

    /// Remove administrator access (legacy API)
    Revoke {
        /// Admin ID
        admin: String,
    },

    /// Update administrator role (legacy API)
    Update {
        /// Admin ID
        admin: String,

        /// New role
        #[arg(long, required = true)]
        role: String,
    },
}
