use serde_derive::{Deserialize, Serialize};

use linked_hash_map::LinkedHashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum IntegerOrString<T> {
    String(String),
    Integer(T),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileConfig {
    pub parts: LinkedHashMap<String, Part>,
    pub files: LinkedHashMap<String, Vec<File>>,
    pub scripts: Option<Scripts>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Part {
    pub value: IntegerOrString<u64>,
    pub factory: Factory,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "name", content = "payload", rename_all = "lowercase")]
pub enum Factory {
    Increment(Option<IncrementPayload>),
    Loop(Vec<IntegerOrString<u64>>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub version: FileVersion,
    pub replaces_count: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileVersion {
    pub view: String,
    pub placement: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scripts {
    pub after_replacement: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IncrementPayload {
    pub default: Option<u64>,
}
