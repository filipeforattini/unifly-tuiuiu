use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct AlarmsArgs {
    #[command(subcommand)]
    pub command: AlarmsCommand,
}

#[derive(Debug, Subcommand)]
pub enum AlarmsCommand {
    /// List alarms (legacy API)
    #[command(alias = "ls")]
    List {
        /// Only show unarchived alarms
        #[arg(long)]
        unarchived: bool,

        /// Max results
        #[arg(long, short = 'l', default_value = "100")]
        limit: u32,
    },

    /// Archive a single alarm (legacy API)
    Archive {
        /// Alarm ID
        id: String,
    },

    /// Archive all alarms (legacy API)
    ArchiveAll,
}
