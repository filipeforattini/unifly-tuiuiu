use clap::{Args, Subcommand};

use super::ListArgs;

#[derive(Debug, Args)]
pub struct VpnArgs {
    #[command(subcommand)]
    pub command: VpnCommand,
}

#[derive(Debug, Subcommand)]
pub enum VpnCommand {
    /// List VPN servers
    Servers(ListArgs),

    /// List site-to-site VPN tunnels
    Tunnels(ListArgs),
}
