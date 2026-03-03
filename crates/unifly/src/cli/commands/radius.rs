//! RADIUS profile command handlers.

use tabled::Tabled;
use unifly_api::{Controller, RadiusProfile};

use crate::cli::args::{GlobalOpts, RadiusArgs, RadiusCommand};
use crate::cli::error::CliError;
use crate::cli::output;

use super::util;

// ── Table row ───────────────────────────────────────────────────────

#[derive(Tabled)]
struct RadiusProfileRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
}

impl From<&RadiusProfile> for RadiusProfileRow {
    fn from(r: &RadiusProfile) -> Self {
        Self {
            id: r.id.to_string(),
            name: r.name.clone(),
        }
    }
}

// ── Handler ─────────────────────────────────────────────────────────

pub async fn handle(
    controller: &Controller,
    args: RadiusArgs,
    global: &GlobalOpts,
) -> Result<(), CliError> {
    match args.command {
        RadiusCommand::Profiles(list) => {
            let profiles = util::apply_list_args(
                controller.list_radius_profiles().await?,
                &list,
                util::matches_json_filter,
            );
            let out = output::render_list(
                &global.output,
                &profiles,
                |r| RadiusProfileRow::from(r),
                |r| r.id.to_string(),
            );
            output::print_output(&out, global.quiet);
            Ok(())
        }
    }
}
