use clap;

mod add;
mod remove;

use crate::context::RTContext;
use crate::handleable::{CmdResult, Handleable};

#[derive(Debug, clap::Subcommand)]
pub enum ProfileCommands {
    Add(add::Add),
}

/// Manage your profiles
#[derive(Debug, clap::Args)]
pub struct Profile {
    #[clap(subcommand)]
    command: ProfileCommands,
}

impl Handleable for Profile {
    fn handle(&self, ctx: &mut RTContext) -> CmdResult {
        match &self.command {
            ProfileCommands::Add(inst) => inst.handle(ctx),
        }
    }
}
