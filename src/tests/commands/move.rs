#![cfg(test)]

use std::io::Write;

use clap::Parser;
use serial_test::serial;

use crate::commands::CLI;
use crate::tests::utils::simple_project::{SimpleProject, SimpleProjectVersions};

#[cfg(test)]
mod simple_project {

    use super::*;

    #[cfg(test)]
    mod pyproject {

        use super::*;

        #[test]
        #[serial]
        fn move_custom() {
            let project = SimpleProject::setup();
            let mut child = std::process::Command::new("cargo")
                .arg("run")
                .arg("--")
                .arg("move")
                .arg("project")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .unwrap();

            let child_stdin = child.stdin.as_mut().unwrap();
            child_stdin
                .write_all(b"40\n3\n8\nalpha\na\n-1\n14\n")
                .unwrap();
            // Close stdin to finish and avoid indefinite blocking
            drop(child_stdin);
            child.wait_with_output().unwrap();

            assert_eq!(
                SimpleProjectVersions {
                    project: "40.3.8a14".into(),
                    dep: "0.1.0-alpha0".into(),
                    dep_another_style: "0.1".into()
                },
                project.fetch_versions()
            );
        }
    }
}
