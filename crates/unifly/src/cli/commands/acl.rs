//! ACL rule command handlers.

mod handler;
mod render;
mod request;

use unifly_api::Controller;

use crate::cli::args::{AclArgs, GlobalOpts};
use crate::cli::error::CliError;

pub async fn handle(
    controller: &Controller,
    args: AclArgs,
    global: &GlobalOpts,
) -> Result<(), CliError> {
    handler::handle(controller, args, global).await
}
