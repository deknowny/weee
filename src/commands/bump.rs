use clap;
use colored::Colorize;

use crate::handleable::{CmdResult, Handleable};
use crate::context::RTContext;
use crate::config::IntegerOrString;

#[derive(Debug, clap::Args)]
pub struct Bump {
    #[clap(required = true)]
    profile: String,

    #[clap(required = true)]
    version_part: String,

    #[clap(long)]
    read_only: bool,
}

impl Handleable for Bump {
    fn handle(&self, ctx: &mut RTContext) -> CmdResult {
        let mut profile_ctx = ctx.fetch_profile_conext(&self.profile)?;
        let changed_version = profile_ctx.bump_version(&self.version_part)?;


        println!(
            " \u{1F389} Weee! Bumping {} ({} -> {})",
            self.version_part.cyan(),
            match &changed_version.old[&self.version_part] {
                IntegerOrString::Integer(val) => val.to_string(),
                IntegerOrString::String(val) => val.to_string()
            }.red(),
            match &changed_version.new[&self.version_part] {
                IntegerOrString::Integer(val) => val.to_string(),
                IntegerOrString::String(val) => val.to_string()
            }.green(),
        );
        let prepared_changed_files = profile_ctx.prepare_replacemts(&changed_version)?;
        profile_ctx.change_files_content(&prepared_changed_files, self.read_only)?;
        profile_ctx.update_storage(&changed_version, self.read_only)?;


        Ok(())
    }
}
