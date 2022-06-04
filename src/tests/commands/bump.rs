#![cfg(test)]

use clap::Parser;
use serial_test::serial;

use crate::tests::utils::simple_project::{SimpleProject, SimpleProjectVersions};
use crate::commands::CLI;


#[cfg(test)]
mod simple_project {

    use super::*;

    #[test]
    #[serial]
    fn bump_major() {
        let project = SimpleProject::setup();
        CLI::parse_from(["weee", "bump", "project", "major"]).handle();
        assert_eq!(
            SimpleProjectVersions {
                project: "15.0.0a0".into(),
                dep: "0.1.0-alpha0".into(),
                dep_another_style: "0.1".into()
            },
            project.fetch_versions()
        );
    }

    #[test]
    #[serial]
    fn bump_minor() {
        let project = SimpleProject::setup();
        CLI::parse_from(["weee", "bump", "project", "minor"]).handle();
        assert_eq!(
            SimpleProjectVersions {
                project: "14.24.0a0".into(),
                dep: "0.1.0-alpha0".into(),
                dep_another_style: "0.1".into()
            },
            project.fetch_versions()
        );
    }

    #[test]
    #[serial]
    fn bump_patch() {
        let project = SimpleProject::setup();
        CLI::parse_from(["weee", "bump", "project", "patch"]).handle();
        assert_eq!(
            SimpleProjectVersions {
                project: "14.23.5646a0".into(),
                dep: "0.1.0-alpha0".into(),
                dep_another_style: "0.1".into()
            },
            project.fetch_versions()
        );
    }

    #[test]
    #[serial]
    fn bump_stage() {
        let project = SimpleProject::setup();
        CLI::parse_from(["weee", "bump", "project", "stage"]).handle();
        assert_eq!(
            SimpleProjectVersions {
                project: "14.23.5646a0".into(),
                dep: "0.1.0-alpha0".into(),
                dep_another_style: "0.1".into()
            },
            project.fetch_versions()
        );
    }

    #[test]
    #[serial]
    fn overflow_stage() {
        let project = SimpleProject::setup();
        CLI::parse_from(["weee", "bump", "project", "stage"]).handle();
        assert_eq!(
            SimpleProjectVersions {
                project: "14.23.5646a0".into(),
                dep: "0.1.0-alpha0".into(),
                dep_another_style: "0.1".into()
            },
            project.fetch_versions()
        );

        CLI::parse_from(["weee", "bump", "project", "stage"]).handle();
        assert_eq!(
            SimpleProjectVersions {
                project: "14.23.5646b0".into(),
                dep: "0.1.0-alpha0".into(),
                dep_another_style: "0.1".into()
            },
            project.fetch_versions()
        );

        CLI::parse_from(["weee", "bump", "project", "step"]).handle();
        assert_eq!(
            SimpleProjectVersions {
                project: "14.23.5646b1".into(),
                dep: "0.1.0-alpha0".into(),
                dep_another_style: "0.1".into()
            },
            project.fetch_versions()
        );

        CLI::parse_from(["weee", "bump", "project", "stage"]).handle();
        assert_eq!(
            SimpleProjectVersions {
                project: "14.23.5647a0".into(),
                dep: "0.1.0-alpha0".into(),
                dep_another_style: "0.1".into()
            },
            project.fetch_versions()
        );
    }
}
