#![cfg(test)]
use fs_extra;
use toml_edit::Document;

#[cfg(not(target_os = "windows"))]
const SIMPLE_PROJECT_PATH: &'static str = "src/tests/projects/simple";

#[cfg(target_os = "windows")]
const SIMPLE_PROJECT_PATH: &'static str = r#"src\tests\projects\simple"#;

pub struct SimpleProject {
    pub path: std::path::PathBuf,
}

#[derive(Debug, PartialEq)]
pub struct SimpleProjectVersions {
    pub dep: String,
    pub dep_another_style: String,
    pub project: String,
}

impl SimpleProject {
    pub fn setup() -> SimpleProject {
        let temp_dir = std::env::temp_dir();
        let project_path = temp_dir.join("simple");
        std::env::set_var("WEEE_PROJECT_PATH", &project_path);
        let options = fs_extra::dir::CopyOptions::new();

        // For non-existed files
        std::fs::remove_dir_all(&project_path).unwrap_or(());

        fs_extra::dir::copy(SIMPLE_PROJECT_PATH, &temp_dir, &options)
            .expect("Failed to copy test directory to temporary directory and run tests");
        SimpleProject { path: project_path }
    }
}

impl SimpleProject {
    pub fn fetch_versions(&self) -> SimpleProjectVersions {
        dbg!(&self.path);
        let req = std::fs::read_to_string(&self.path.join("req.txt"))
            .expect("Cannot read req.txt content");
        let req = req.trim_end();

        let (first, last) = req.split_once("\n").expect("Invalid req.txt content");

        let first = first
            .split_once("==")
            .expect("Invalid version separator for req.txt")
            .1;

        let last = last
            .split_once("==")
            .expect("Invalid version separator for req.txt")
            .1;

        let pyproject = std::fs::read_to_string(&self.path.join("pyproject.toml"))
            .expect("Cannot read req.txt content");

        let pyproject_doc = pyproject
            .parse::<Document>()
            .expect("Invalid TOML syntax for pyproject.toml");

        let pyproject_version = match &pyproject_doc["project"]["version"] {
            toml_edit::Item::Value(val) => match val {
                toml_edit::Value::String(val) => val,
                _ => unreachable!("Invalid type for version in pyproject.toml"),
            },
            _ => unreachable!("Invalid type for version in pyproject.toml"),
        };

        SimpleProjectVersions {
            dep: first.to_string(),
            dep_another_style: last.to_string(),
            project: pyproject_version.value().clone(),
        }
    }
}
