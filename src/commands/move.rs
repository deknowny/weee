use clap;

use linked_hash_map::LinkedHashMap;

use crate::context::{ChangedVersion, RTContext};
use crate::handleable::{CmdResult, Handleable};

/// Move profile's version to custom value
#[derive(Debug, clap::Args)]
pub struct Move {
    /// Profile that would be used
    #[clap(required = true)]
    profile: String,

    /// Do not any changes in files,
    /// only show how it would be changed
    #[clap(long)]
    read_only: bool,
}

impl Handleable for Move {
    fn handle(&self, ctx: &mut RTContext) -> CmdResult {
        let mut profile_ctx = ctx.fetch_profile_conext(&self.profile)?;

        // Todo: show which parts were modified
        println!(" \u{1F389} Weee! Bumping to custom version. Let's decide how the new version should look like!");
        let new_version = profile_ctx.ask_another_version()?;

        println!("");
        if self.read_only {
            println!(" \u{1F44D} Well, now going to do changes (read-only)...");
        } else {
            println!(" \u{1F44D} Well, now going to do changes...");
        }

        let mut old_version = LinkedHashMap::new();
        for (part_name, part_info) in profile_ctx.profile_model.parts.iter() {
            old_version.insert(part_name.clone(), part_info.value.clone());
        }
        let changed_version = ChangedVersion {
            new: new_version,
            old: old_version,
        };

        let prepared_changed_files = profile_ctx.prepare_replacemts(&changed_version)?;
        profile_ctx.change_files_content(&prepared_changed_files, self.read_only)?;
        profile_ctx.update_storage(&changed_version, self.read_only)?;

        Ok(())
    }
}
