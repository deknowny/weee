use toml;
use serde_derive::{Serialize, Deserialize};

use linked_hash_map::{LinkedHashMap};

use crate::handleable::CmdResult;
use crate::show_err;


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum IntegerOrString<T> {
    String(String),
    Integer(T)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileConfig {
    pub parts: LinkedHashMap<String, Part>,
    pub files: LinkedHashMap<String, Vec<Part>>,
    pub scripts: Option<Scripts>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Part {
    pub value: IntegerOrString<u64>,
    pub factory: Factory
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "name", content = "payload", rename_all = "lowercase")]
pub enum Factory {
    Increment { default: Option<u64> },
    Loop(Vec<IntegerOrString<u64>>)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub version: Version
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub view: String,
    pub placement: String,
    pub replaces_count: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scripts {
    pub after_replacement: Option<String>
}
