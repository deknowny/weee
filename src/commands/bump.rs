use clap;

use crate::handleable::{CmdResult, Handleable};
use crate::context::RTContext;

#[derive(Debug, clap::Args)]
pub struct Bump {
    #[clap(required = true)]
    profile: String,

    #[clap(required = true)]
    version_part: String,
}

impl Handleable for Bump {
    fn handle(&self, ctx: &mut RTContext) -> CmdResult {
        let profile_ctx = ctx.fetch_profile_conext(&self.profile)?;
        let new_version = profile_ctx.bump_version(&self.version_part)?;
        dbg!(new_version);
        Ok(())
    }
}
