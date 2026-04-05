use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct AdminArgs {
    #[command(subcommand)]
    pub command: AdminCommand,
}

#[derive(Debug, Subcommand)]
pub enum AdminCommand {
    /// List site administrators (session API)
    #[command(alias = "ls")]
    List,

    /// Invite a new administrator (session API)
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

    /// Remove administrator access (session API)
    Revoke {
        /// Admin ID
        admin: String,
    },

    /// Update administrator role (session API)
    Update {
        /// Admin ID
        admin: String,

        /// New role
        #[arg(long, required = true)]
        role: String,
    },
}
