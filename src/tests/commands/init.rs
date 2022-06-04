#![cfg(test)]

use clap::Parser;
use serial_test::serial;

use crate::commands::CLI;
use crate::tests::utils::simple_project::SimpleProject;

#[test]
#[serial]
fn default_initing() {
    let project = SimpleProject::setup();
    std::fs::remove_dir_all(project.path.join(".weee")).unwrap_or(());

    CLI::parse_from(["weee", "init"]).handle();

    assert_eq!(project.path.join(".weee").exists(), true);
    assert_eq!(
        project
            .path
            .join(".weee")
            .join("project.version.toml")
            .exists(),
        true
    );
}

#[test]
#[serial]
fn initing_with_custom_profile_name() {
    let project = SimpleProject::setup();
    std::fs::remove_dir_all(project.path.join(".weee")).unwrap_or(());

    CLI::parse_from(["weee", "init", "-p", "some"]).handle();

    assert_eq!(project.path.join(".weee").exists(), true);
    assert_eq!(
        project
            .path
            .join(".weee")
            .join("some.version.toml")
            .exists(),
        true
    );
}
