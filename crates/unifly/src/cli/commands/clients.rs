//! Client command handlers.

mod handler;
mod render;
mod resolve;

use unifly_api::Controller;

use crate::cli::args::{ClientsArgs, GlobalOpts};
use crate::cli::error::CliError;

pub async fn handle(
    controller: &Controller,
    args: ClientsArgs,
    global: &GlobalOpts,
) -> Result<(), CliError> {
    handler::handle(controller, args, global).await
}
