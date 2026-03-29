//! Firewall command handlers (policies + zones).

mod policies;
mod shared;
mod zones;

use unifly_api::Controller;

use crate::cli::args::{FirewallArgs, FirewallCommand, GlobalOpts};
use crate::cli::error::CliError;
use crate::cli::output;

use super::util;

pub async fn handle(
    controller: &Controller,
    args: FirewallArgs,
    global: &GlobalOpts,
) -> Result<(), CliError> {
    util::ensure_integration_access(controller, "firewall").await?;
    let painter = output::Painter::new(global);

    match args.command {
        FirewallCommand::Policies(args) => {
            policies::handle(controller, args.command, global, &painter).await
        }
        FirewallCommand::Zones(args) => {
            zones::handle(controller, args.command, global, &painter).await
        }
    }
}
