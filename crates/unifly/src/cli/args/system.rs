use std::path::PathBuf;

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct SystemArgs {
    #[command(subcommand)]
    pub command: SystemCommand,
}

#[derive(Debug, Subcommand)]
pub enum SystemCommand {
    /// Application version info (Integration API)
    Info,

    /// Site health summary (session API)
    Health,

    /// Controller system info (session API)
    Sysinfo,

    /// Backup management (session API)
    Backup(BackupArgs),

    /// Reboot controller hardware (session API, UDM only)
    Reboot,

    /// Power off controller hardware (session API, UDM only)
    Poweroff,
}

#[derive(Debug, Args)]
pub struct BackupArgs {
    #[command(subcommand)]
    pub command: BackupCommand,
}

#[derive(Debug, Subcommand)]
pub enum BackupCommand {
    /// Create a new backup
    Create,

    /// List existing backups
    #[command(alias = "ls")]
    List,

    /// Download a backup file
    Download {
        /// Backup filename
        filename: String,

        /// Destination path (default: current directory)
        #[arg(long = "path")]
        path: Option<PathBuf>,
    },

    /// Delete a backup
    Delete {
        /// Backup filename
        filename: String,
    },
}
