use std;
use std::io::Error;

use crate::handleable::{CmdResult, CLIError};
use crate::show_err;


pub struct RTContext {}


// initializing
impl RTContext {
    pub fn new() -> Self {
        RTContext {}
    }
}


// Working with files creation/deleting
impl RTContext {
    pub fn create_weee_dir(&self) -> CmdResult {
        let weee_dir_path = std::path::Path::new(".weee");
        if let Err(err) = std::fs::create_dir(&weee_dir_path) {
            return match err.kind() {
                std::io::ErrorKind::AlreadyExists => show_err!(
                    [WeeeDirectoryAlreadyExists]
                    => "Weee tool has been already initialized. It's ready to use"
                ),
                _ => show_err!(
                    [WeeeDirectoryAlreadyExists]
                    => "An OS error occured while creating a .weee directory",
                    os_error=err
                )
            };
        }
        Ok(())
    }

    pub fn create_weee_profile(&self, name: &str) -> CmdResult {
        let weee_dir_path = std::path::Path::new(".weee");
        let rules_path = weee_dir_path.join(format!("{}.rules.toml", name));
        let storage_path = weee_dir_path.join(format!("{}.storage.json", name));

        if rules_path.exists() {
            return show_err!(
                [ProfileRuleAlreadyExists]
                => "Such profile rule has been already created",
                profile_name=name
            );
        }
        if storage_path.exists() {
            return show_err!(
                [ProfileRuleAlreadyExists]
                => "Such profile storage has been already created",
                profile_name=name
            );
        }

        let rules_file = std::fs::File::create(&rules_path);
        if let Err(err) = rules_file {
            return match err.kind() {
                std::io::ErrorKind::AlreadyExists => show_err!(
                    [ProfileRuleAlreadyExists]
                    => "Such profile rule already exists",
                    profile_rule_file=rules_path.display()
                ),
                _ => show_err!(
                    [CannotCreateProfileRule]
                    => "An OS error occured while creating rule for the profile",
                    os_error=err,
                    profile=name
                )
            };
        }

        let storage_file = std::fs::File::create(&storage_path);
        if let Err(err) = storage_file {
            return match err.kind() {
                std::io::ErrorKind::AlreadyExists => show_err!(
                    [ProfileStorageAlreadyExists]
                    => "Such profile storage already exists",
                    profile_storage_file=storage_path.display()
                ),
                _ => show_err!(
                    [CannotCreateProfileStorage]
                    => "An OS error occured while creating storage for the profile",
                    os_error=err,
                    profile=name
                )
            };
        }

        Ok(())
    }
}
