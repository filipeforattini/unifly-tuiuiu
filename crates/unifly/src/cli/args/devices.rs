use clap::{Args, Subcommand};

use super::ListArgs;

#[derive(Debug, Args)]
pub struct DevicesArgs {
    #[command(subcommand)]
    pub command: DevicesCommand,
}

#[derive(Debug, Subcommand)]
pub enum DevicesCommand {
    /// List adopted devices
    #[command(alias = "ls")]
    List(ListArgs),

    /// Get adopted device details
    Get {
        /// Device ID (UUID) or MAC address
        device: String,
    },

    /// Adopt a pending device
    Adopt {
        /// MAC address of the device to adopt
        #[arg(value_name = "MAC")]
        mac: String,

        /// Ignore device limit on the site
        #[arg(long)]
        ignore_limit: bool,
    },

    /// Remove (unadopt) a device
    Remove {
        /// Device ID (UUID) or MAC address
        device: String,
    },

    /// Restart a device
    Restart {
        /// Device ID (UUID) or MAC address
        device: String,
    },

    /// Toggle locate LED (blink to identify device)
    Locate {
        /// Device MAC address
        device: String,

        /// Turn locate on (default) or off
        #[arg(long, default_value = "true", action = clap::ArgAction::Set)]
        on: bool,
    },

    /// Power-cycle a PoE port
    PortCycle {
        /// Device ID (UUID) or MAC address
        device: String,

        /// Port index to power-cycle
        #[arg(value_name = "PORT_IDX")]
        port: u32,
    },

    /// Get real-time device statistics
    Stats {
        /// Device ID (UUID) or MAC address
        device: String,
    },

    /// List devices pending adoption
    Pending(ListArgs),

    /// Upgrade device firmware (legacy API)
    Upgrade {
        /// Device MAC address
        device: String,

        /// External firmware URL (optional)
        #[arg(long)]
        url: Option<String>,
    },

    /// Force re-provision device configuration (legacy API)
    Provision {
        /// Device MAC address
        device: String,
    },

    /// Run WAN speed test (legacy API, gateway only)
    Speedtest,

    /// List device tags
    Tags(ListArgs),
}
