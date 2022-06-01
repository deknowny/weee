use clap;

use crate::context::RTContext;
use crate::handleable::{CmdResult, Handleable};

#[derive(Debug, clap::Args)]
pub struct Add {
    #[clap(required = true)]
    profile_name: String,
}

impl Handleable for Add {
    fn handle(&self, ctx: &mut RTContext) -> CmdResult {
        ctx.create_weee_profile(&self.profile_name)
    }
}
