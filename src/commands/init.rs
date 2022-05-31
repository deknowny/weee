use std;

use clap;

use crate::error::CLIError;
use crate::handleable::{CmdResult, Handleable};
use crate::rtcontext::RTContext;
use crate::show_err;

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
