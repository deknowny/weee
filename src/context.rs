use std;

use toml_edit::Document;
use linked_hash_map::LinkedHashMap;

use crate::error::CLIError;
use crate::handleable::CmdResult;
use crate::show_err;
use crate::config::{ProfileConfig, Factory, Part, IntegerOrString};


type Version = std::collections::HashMap<String, IntegerOrString<u64>>;


pub struct RTContext {
    base_path: std::ffi::OsString,
}

// initializing
impl RTContext {
    pub fn new() -> Self {
        RTContext {
            base_path: match std::env::var_os("WEEE_PROJECT_PATH") {
                Some(val) => val,
                None => std::ffi::OsString::from("."),
            },
        }
    }
}

// Working with files creation/deleting
impl RTContext {
    pub fn create_weee_dir(&self) -> CmdResult {
        let weee_dir_path = std::path::Path::new(&self.base_path).join(".weee");
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
                ),
            };
        }
        Ok(())
    }

    pub fn create_weee_profile(&self, name: &str) -> CmdResult {
        let weee_dir_path = std::path::Path::new(".weee");
        let rules_path = weee_dir_path.join(format!("{}.version.toml", name));

        if rules_path.exists() {
            return show_err!(
                [ProfileAlreadyExists]
                => "Such version profile config already exists",
                file=rules_path.display()
            );
        }

        let rules_file = std::fs::File::create(&rules_path);
        if let Err(err) = rules_file {
            return show_err!(
                [CannotCreateProfileRule]
                => "An OS error occured while creating rule for the profile",
                os_error=err,
                profile=name
            );
        }

        Ok(())
    }
}


impl RTContext {
    pub fn fetch_profile_conext(&self, profile: &str) -> CmdResult<ProfileContext> {
        ProfileContext::load(self, profile)
    }

}

pub struct ProfileContext<'rtctx> {
    pub rt_context: &'rtctx RTContext,
    pub profile_model: ProfileConfig,
    pub profile_doc: Document,
}


impl<'rtctx> ProfileContext<'rtctx> {
    pub fn load(rt_context: &'rtctx RTContext, profile: &str) -> CmdResult<Self> {
        let filename = format!("{}.version.toml", profile);
        let profile_path = std::path::Path::new(&rt_context.base_path).join(".weee").join(filename);

        let profile_content = match std::fs::read_to_string(&profile_path) {
            Ok(content) => content,
            Err(err) => return match err.kind() {
                std::io::ErrorKind::NotFound => show_err!(
                    [NoSuchProfileExists]
                    => "No such profile exists",
                    profile_path=match profile_path.to_str() {
                        Some(val) => val,
                        None => ".."
                    }
                ),
                _ => show_err!(
                    [CannotCreateProfileRule]
                    => "An OS error occured while creating rule for the profile",
                    os_err=err
                ),
            }
        };

        let profile_model = match toml::from_str::<ProfileConfig>(&profile_content) {
            Ok(model) => model,
            Err(err) => panic!("{:#?}", err)
        };

        let profile_doc = match profile_content.parse::<Document>() {
            Ok(model) => model,
            Err(err) => panic!("{:#?}", err)
        };

        Ok(
            ProfileContext {
                rt_context,
                profile_model,
                profile_doc
            }
        )
    }
}

// Checks
impl<'rtctx> ProfileContext<'rtctx> {
    pub fn check_part_exists(&self, part: &str) -> CmdResult<Part> {
        match self.profile_model.parts.get(part) {
            Some(val) => Ok((*val).clone()),
            None => show_err!(
                [NoSuchVersionPartExists]
                => "Such version part does not exist"
            )
        }
    }

    pub fn check_part_is_incrementable(&self, part: &str) -> CmdResult<u64> {
        match self.check_part_exists(part)?.value {
            IntegerOrString::Integer(val) => Ok(val),
            IntegerOrString::String(val) => match val.parse::<u64>() {
                Ok(val) => Ok(val),
                Err(err) => show_err!(
                    [CannotCastVersionValueToIntger]
                    => "Version part is not a valid integer"
                )
            }
        }
    }

    pub fn fetch_default_of_part(&self, part: &str) -> CmdResult<IntegerOrString<u64>> {
        let existed_part = self.check_part_exists(part)?;
        match existed_part.factory {
            Factory::Increment { default } => Ok(IntegerOrString::Integer(default.unwrap_or_default())),
            Factory::Loop(chain) => match chain.get(0) {
                None => show_err!(
                    [LoopFactoryPayloadIsEmpty]
                    => "Loop factory payload cannot be empty"
                ),
                Some(val) => Ok((*val).clone())

            },
        }
    }

    pub fn fetch_next_of_part(&self, part: &str) -> CmdResult<IntegerOrString<u64>> {
        let existed_part = self.check_part_exists(part)?;
        match existed_part.factory {
            Factory::Increment { default } => match existed_part.value {
                IntegerOrString::Integer(val) => Ok(IntegerOrString::Integer(val + 1)),
                IntegerOrString::String(val) => match val.parse::<u64>() {
                    Err(err) => show_err!(
                        [CannotParsePartValueToInteger]
                        => "Version part value is not a valid integer",
                    ),
                    Ok(val) => Ok(IntegerOrString::Integer(val + 1))
                }
            },
            Factory::Loop(chain) => {
                let mut chain_iter = chain.iter();
                for val in chain.iter() {
                    if *val == existed_part.value {
                        return match chain_iter.next() {
                            None => self.fetch_default_of_part(part),
                            Some(val) => Ok((*val).clone())
                        }
                    }
                }
                return show_err!(
                    [CurrentValueOfLoopedPartDoesNotExist]
                    => "Version part managed by a loop factory with no such part in payload"
                )
            }
        }
    }

    // pub fn check_factories(&self) -> CmdResult {
    //     for (part_name, part_info) in self.profile_model.parts.iter().rev() {
    //         match part_info.factory.name.as_str() {
    //             // No checks for increment factory
    //             "increment" => (),

    //             // Loop factory should have a paylaod as array of u64 or strings
    //             "loop" => match part_info.factory.payload {
    //                 None => show_err!(
    //                     [FactoryNeedsPayload]
    //                     => "Loop factory needs a payload that's been iterated over. Add it like `factory.payload = [\"alpha\", \"beta\"]`"
    //                 ),
    //                 Some(payload) => match payload {

    //                 }
    //             }

    //         }
    //     }
    //     Ok(())
    // }

    pub fn bump_version(&self, requested_part: &str) -> CmdResult<Version> {
        let part_info = self.check_part_exists(requested_part)?;
        let mut new_version = std::collections::HashMap::new();

        match part_info.factory {
            Factory::Increment { default } => {
                let mut self_skipped = false;
                for (part_name, part_info) in self.profile_model.parts.iter() {
                    if part_name == requested_part {
                        self_skipped = true;
                        new_version.insert((*part_name).clone(), self.fetch_default_of_part(part_name)?);
                        continue;
                    } else if self_skipped {
                        new_version.insert((*part_name).clone(), self.fetch_default_of_part(part_name)?);
                    } else {
                        new_version.insert((*part_name).clone(), self.fetch_next_of_part(part_name)?);
                    }
                }
            },
            Factory::Loop(chain) => {
                let mut self_skipped = false;
                let mut previous_overflowed = false;
                for (part_name, part_info) in self.profile_model.parts.iter().rev() {
                    if part_name == requested_part {
                        self_skipped = true;
                        let self_part_next = self.fetch_next_of_part(part_name)?;
                        if self_part_next == self.fetch_default_of_part(part_name)? {
                            previous_overflowed = true;
                        }
                        new_version.insert((*part_name).clone(), self.fetch_default_of_part(part_name)?);
                        continue;
                    } else if self_skipped {
                        if previous_overflowed {
                            let new_value = self.fetch_next_of_part(part_name)?;
                            if new_value == self.fetch_default_of_part(part_name)? {
                                new_version.insert((*part_name).clone(), new_value);
                            } else {
                                previous_overflowed = false;
                                new_version.insert((*part_name).clone(), part_info.value.clone());
                            }
                        } else {
                            new_version.insert((*part_name).clone(), part_info.value.clone());
                        }
                    } else {
                        new_version.insert((*part_name).clone(), self.fetch_default_of_part(part_name)?);
                    }
                }
            }
        }
        Ok(new_version)
    }
}
