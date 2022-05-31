use clap;

use crate::handleable::{CmdResult, Handleable};
use crate::rtcontext::RTContext;

#[derive(Debug, clap::Args)]
pub struct Init {
    #[clap(long, short)]
    profile: Option<String>,
}

impl Handleable for Init {
    fn handle(&self, ctx: &mut RTContext) -> CmdResult {
        ctx.create_weee_dir()?;
        let profile_name = match &self.profile {
            Some(name) => name.as_str(),
            None => "main",
        };
        ctx.create_weee_profile(profile_name)?;
        Ok(())
    }
}
