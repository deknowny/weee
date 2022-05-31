use clap;

use crate::handleable::{CmdResult, Handleable};
use crate::rtcontext::RTContext;

#[derive(Debug, clap::Args)]
pub struct Bump {
    #[clap(required = true)]
    version_part: String,
}

impl Handleable for Bump {
    fn handle(&self, ctx: &mut RTContext) -> CmdResult {
        Ok(())
    }
}
