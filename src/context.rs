use std;
use std::io::Write;

use colored::Colorize;
use linked_hash_map::LinkedHashMap;
use toml_edit::Document;

use crate::config::{Factory, IntegerOrString, Part, ProfileConfig};
use crate::error::CLIError;
use crate::handleable::CmdResult;
use crate::show_err;

type Version = LinkedHashMap<String, IntegerOrString<u64>>;

#[derive(Clone, Debug)]
pub struct ChangedVersion {
    pub old: Version,
    pub new: Version,
}

#[derive(Clone, Debug)]
pub struct ChangedFile {
    pub name: String,
    pub old_part: String,
    pub old_version: String,
    pub new_part: String,
    pub new_version: String,
}

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
    pub profile_name: String,
}

impl<'rtctx> ProfileContext<'rtctx> {
    pub fn load(rt_context: &'rtctx RTContext, profile: &str) -> CmdResult<Self> {
        let filename = format!("{}.version.toml", profile);
        let profile_path = std::path::Path::new(&rt_context.base_path)
            .join(".weee")
            .join(filename);

        let profile_content = match std::fs::read_to_string(&profile_path) {
            Ok(content) => content,
            Err(err) => {
                return match err.kind() {
                    std::io::ErrorKind::NotFound => show_err!(
                        [NoSuchProfileExists]
                        => "No such profile exists. Check out whether you typed scope wrongly or deleted the profile file",
                        profile=profile,
                        path=match profile_path.to_str() {
                            Some(val) => val,
                            None => "<unable to render path>"
                        }
                    ),
                    _ => show_err!(
                        [CannotCreateProfileRule]
                        => "An OS error occured while creating rule for the profile",
                        os_err=err,
                        profile=profile
                    ),
                }
            }
        };

        let profile_doc = match profile_content.parse::<Document>() {
            Ok(model) => model,
            Err(err) => {
                return show_err!(
                    [TOMLInvalidSyntax]
                    => "Invalid syntax in a profile configuration file",
                    path=profile_path.to_str().unwrap_or("<unable to render path>"),
                    error=err
                )
            }
        };

        let profile_model = match toml::from_str::<ProfileConfig>(&profile_content) {
            Ok(model) => model,
            Err(_err) => unreachable!("It's a bug, please, report"),
        };

        Ok(ProfileContext {
            rt_context,
            profile_model,
            profile_doc,
            profile_name: String::from(profile),
        })
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
            ),
        }
    }

    pub fn fetch_default_of_part(&self, part: &str) -> CmdResult<IntegerOrString<u64>> {
        let existed_part = self.check_part_exists(part)?;
        match existed_part.factory {
            Factory::Increment(payload) => Ok(IntegerOrString::Integer(match payload {
                Some(payload) => payload.default.unwrap_or_default(),
                None => 0,
            })),
            Factory::Loop(chain) => match chain.get(0) {
                None => show_err!(
                    [LoopFactoryPayloadIsEmpty]
                    => "Loop factory payload cannot be empty"
                ),
                Some(val) => Ok((*val).clone()),
            },
        }
    }

    pub fn fetch_next_of_part(&self, part: &str) -> CmdResult<IntegerOrString<u64>> {
        let existed_part = self.check_part_exists(part)?;
        match existed_part.factory {
            Factory::Increment(_payload) => match existed_part.value {
                IntegerOrString::Integer(val) => Ok(IntegerOrString::Integer(val + 1)),
                IntegerOrString::String(val) => match val.parse::<u64>() {
                    Err(_err) => show_err!(
                        [CannotParsePartValueToInteger]
                        => "Version part value is not a valid integer",
                    ),
                    Ok(val) => Ok(IntegerOrString::Integer(val + 1)),
                },
            },
            Factory::Loop(chain) => {
                for (pos, elem) in chain.iter().enumerate() {
                    if *elem == existed_part.value {
                        return match chain.get(pos + 1) {
                            None => self.fetch_default_of_part(part),
                            Some(next_val) => Ok((*next_val).clone()),
                        };
                    }
                }
                return show_err!(
                    [CurrentValueOfLoopedPartDoesNotExist]
                    => "Version part managed by a loop factory with no such part in payload"
                );
            }
        }
    }

    fn insert_version_into_string(&self, string: String, version: Version) -> String {
        let mut new_string = string.clone();
        for (version_part, version_value) in version.iter() {
            let temp_val_string;
            new_string = new_string.replace(
                format!("{{{}}}", version_part).as_str(),
                match version_value {
                    IntegerOrString::String(val) => val.as_str(),
                    IntegerOrString::Integer(val) => {
                        temp_val_string = (*val).to_string();
                        temp_val_string.as_str()
                    }
                },
            )
        }
        new_string
    }

    pub fn prepare_replacemts(
        &self,
        changed_version: &ChangedVersion,
    ) -> CmdResult<Vec<ChangedFile>> {
        let mut changed_files = vec![];

        for (file_name, file_replacements) in self.profile_model.files.iter() {
            for file_replacement in file_replacements.iter() {
                let old_version = self.insert_version_into_string(
                    file_replacement.version.view.clone(),
                    changed_version.old.clone(),
                );
                let old_part = file_replacement
                    .version
                    .placement
                    .replace("{version}", old_version.as_str());
                let new_version = self.insert_version_into_string(
                    file_replacement.version.view.clone(),
                    changed_version.new.clone(),
                );
                let new_part = file_replacement
                    .version
                    .placement
                    .replace("{version}", new_version.as_str());

                changed_files.push(ChangedFile {
                    name: (*file_name).clone(),
                    old_part,
                    new_part,
                    new_version,
                    old_version,
                });
            }
        }
        Ok(changed_files)
    }

    pub fn bump_version(&self, requested_part: &str) -> CmdResult<ChangedVersion> {
        let part_info = self.check_part_exists(requested_part)?;

        let mut new_version = LinkedHashMap::new();
        let mut old_version = LinkedHashMap::new();

        // Collect old version
        for (part_name, part_info) in self.profile_model.parts.iter() {
            old_version.insert(part_name.clone(), part_info.value.clone());
        }

        // Collect new version
        match part_info.factory {
            Factory::Increment(_payload) => {
                let mut self_skipped = false;
                for (part_name, part_info) in self.profile_model.parts.iter() {
                    if part_name == requested_part {
                        self_skipped = true;
                        new_version
                            .insert((*part_name).clone(), self.fetch_next_of_part(part_name)?);
                        continue;
                    } else if self_skipped {
                        new_version
                            .insert((*part_name).clone(), self.fetch_default_of_part(part_name)?);
                    } else {
                        new_version.insert((*part_name).clone(), part_info.value.clone());
                    }
                }
            }
            Factory::Loop(_chain) => {
                let mut self_skipped = false;
                let mut previous_overflowed = false;
                for (part_name, part_info) in self.profile_model.parts.iter().rev() {
                    if part_name == requested_part {
                        self_skipped = true;
                        let self_part_next = self.fetch_next_of_part(part_name)?;
                        if self_part_next == self.fetch_default_of_part(part_name)? {
                            previous_overflowed = true;
                        }
                        new_version.insert((*part_name).clone(), self_part_next);
                        continue;
                    } else if self_skipped {
                        if previous_overflowed {
                            let new_value = self.fetch_next_of_part(part_name)?;
                            if new_value != self.fetch_default_of_part(part_name)? {
                                previous_overflowed = false;
                            }
                            new_version.insert((*part_name).clone(), new_value);
                        } else {
                            new_version.insert((*part_name).clone(), part_info.value.clone());
                        }
                    } else {
                        new_version
                            .insert((*part_name).clone(), self.fetch_default_of_part(part_name)?);
                    }
                }
            }
        }
        Ok(ChangedVersion {
            new: new_version,
            old: old_version,
        })
    }

    pub fn change_files_content(
        &self,
        changed_files: &Vec<ChangedFile>,
        read_only: bool,
    ) -> CmdResult {
        // One file can have multiply patterns, but this function accepts
        // a vector of changes (i.e. applied pattern changing)
        // So if we count "hits" of filename we can detect
        // which pattern of given is now
        let mut filenames_hits: std::collections::HashMap<&String, usize> =
            std::collections::HashMap::new();

        // If a file has beed changed by abother pattern, we should keep new changes
        let mut changed_files_content: std::collections::HashMap<&String, String> =
            std::collections::HashMap::new();

        for file in changed_files.iter() {
            let splited_path: Vec<&str> = file.name.split("/").collect();
            let mut os_based_file_path =
                std::path::Path::new(&self.rt_context.base_path).join(splited_path[0]);
            for path_part in &splited_path[1..] {
                os_based_file_path = os_based_file_path.join(path_part);
            }

            if !os_based_file_path.exists() {
                return show_err!(
                    [NoSuchFileForReplacements]
                    => "No such file to make version replacements"
                );
            }
            let file_content_string;
            let file_content = match &changed_files_content.get(&file.name) {
                Some(content) => content,
                None => match std::fs::read_to_string(&os_based_file_path) {
                    Ok(content) => {
                        file_content_string = content;
                        &file_content_string
                    }
                    Err(_err) => {
                        return show_err!(
                            [CannotReadReplacementsFileContent]
                            => "Cannot read file to make replacements"
                        )
                    }
                },
            };

            let old_version_matches_count = file_content.matches(&file.old_part).count() as u64;

            let this_file_paterns = &self.profile_model.files[&file.name];
            let current_hits = filenames_hits.entry(&file.name).or_insert(0);

            let new_file_content;
            if let Some(replaces_count) = this_file_paterns[*current_hits].replaces_count {
                if replaces_count < old_version_matches_count {
                    return show_err!(
                        [NotEnoughOldVersionMatches]
                        => "Count of old version entries is not as supposed"
                    );
                }
                new_file_content =
                    file_content.replacen(&file.old_part, &file.new_part, replaces_count as usize);
            } else if old_version_matches_count == 0 {
                return show_err!(
                    [FileDoesNotContainOldVersion]
                    => "Changed files has no old version in it's content",
                    profile=self.profile_name,
                    file=file.name,
                    old_match=&file.old_part,

                );
            } else {
                new_file_content = file_content.replace(&file.old_part, &file.new_part);
            }

            if !read_only {
                if let Err(_err) = std::fs::write(&os_based_file_path, &new_file_content) {
                    return show_err!(
                        [CannotWriteToFile]
                        => "Cannot write new version into file"
                    );
                };
            }

            changed_files_content.insert(&file.name, new_file_content);

            println!(
                "[{}]: {} => {}",
                os_based_file_path
                    .to_str()
                    .unwrap_or("<cannot render path>")
                    .magenta(),
                file.old_version.red(),
                file.new_version.green(),
            );
            *current_hits += 1;
        }

        Ok(())
    }

    pub fn update_storage(
        &mut self,
        changed_version: &ChangedVersion,
        read_only: bool,
    ) -> CmdResult {
        for (part, new_value) in changed_version.new.iter() {
            match new_value {
                IntegerOrString::Integer(val) => {
                    self.profile_doc["parts"][part]["value"] = toml_edit::value(*val as i64)
                }
                IntegerOrString::String(val) => {
                    self.profile_doc["parts"][part]["value"] = toml_edit::value(val.clone())
                }
            };
        }

        if !read_only {
            if let Err(_err) = std::fs::write(
                std::path::Path::new(&self.rt_context.base_path)
                    .join(".weee")
                    .join(format!("{}.version.toml", self.profile_name)),
                self.profile_doc.to_string(),
            ) {
                return show_err!(
                    [CannotWriteToProfileFile]
                    => "An OS error accured while writing to profile file"
                );
            };
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn version_to_string(&self, version: &Version) -> String {
        let mut result_string = String::new();
        for (ind, pair) in version.iter().enumerate() {
            if ind != 0 {
                result_string.push('.');
            }
            match pair.1 {
                IntegerOrString::Integer(val) => {
                    let temp_string = val.to_string();
                    result_string.push_str(temp_string.as_str())
                }
                IntegerOrString::String(val) => result_string.push_str(val),
            };
        }
        result_string
    }
}

impl<'rtctx> ProfileContext<'rtctx> {
    fn show_tip(&self, prompt: &'static str) {
        eprintln!(" \u{1F4A5} Oh! {}", prompt);
    }

    pub fn ask_another_version(&self) -> CmdResult<Version> {
        let mut new_version = LinkedHashMap::new();
        for (part_name, part_info) in self.profile_model.parts.iter() {
            new_version.insert(
                part_name.clone(),
                self.ask_for_part(&part_name, &part_info)?,
            );
        }

        Ok(new_version)
    }

    fn ask_for_part(&self, part_name: &str, part_info: &Part) -> CmdResult<IntegerOrString<u64>> {
        loop {
            let mut new_part_value = String::new();
            print!(
                "[{} {}]: ",
                part_name.magenta(),
                format!(
                    "({})",
                    match &part_info.value {
                        IntegerOrString::Integer(val) => val.to_string(),
                        IntegerOrString::String(val) => val.to_string(),
                    }
                    .yellow()
                )
                .bright_black()
            );
            if let Err(_err) = std::io::stdout().flush() {
                return show_err!(
                    [CannotFlushStdout]
                    => "Cannot flush stdout"
                );
            };
            if let Err(_err) = std::io::stdin().read_line(&mut new_part_value) {
                return show_err!(
                    [CannotReadNewValueFromStdin]
                    => "Cannot get an input for new part value"
                );
            }
            new_part_value.truncate(new_part_value.len() - 1);

            match &part_info.factory {
                Factory::Increment(_payload) => {
                    match new_part_value.parse::<u64>() {
                        Ok(value) => return Ok(
                            IntegerOrString::Integer(value)
                        ),
                        Err(_err) => {
                            self.show_tip(
                                "Cannot treat your input as an integer (this part uses increment factory so it's required to be integer)"
                            )
                        }
                    }
                },
                Factory::Loop(payload) => {
                    for available_part in payload.iter() {
                        let stringified_part = match available_part {
                            IntegerOrString::Integer(val) => val.to_string(),
                            IntegerOrString::String(val) => val.to_string(),
                        };
                        if stringified_part == new_part_value {
                            return Ok(match available_part {
                                IntegerOrString::Integer(_) => IntegerOrString::Integer(
                                    new_part_value.parse::<u64>().expect("It's a bug in type convereting, please, report an issue")
                                ),
                                IntegerOrString::String(val) => IntegerOrString::String(val.clone()),
                            })
                        }
                    }
                    self.show_tip(
                        "No such part is available from loop's factory payload. Check if you typed everything correctly"
                    );
                }
            }
        }
    }
}

// hooks
impl<'rtctx> ProfileContext<'rtctx> {
    fn process_args(
        &self,
        args: &Vec<String>,
        changed_version: &ChangedVersion,
    ) -> CmdResult<Vec<String>> {
        let mut new_args = vec![];
        for (_ind, arg) in args.iter().enumerate() {
            if arg.starts_with("!ASK:") {
                let split_data = arg.split_once(":");
                if let Some((_, prompt)) = split_data {
                    print!("===> [{}]: ", prompt.magenta());
                    if let Err(_err) = std::io::stdout().flush() {
                        return show_err!(
                            [CannotFlushStdout]
                            => "Cannot flush stdout"
                        );
                    };
                    let mut new_value = String::new();
                    if let Err(_err) = std::io::stdin().read_line(&mut new_value) {
                        return show_err!(
                            [CannotReadNewValueFromStdin]
                            => "Cannot get an input for asked value"
                        );
                    }
                    new_value.truncate(new_value.len() - 1);
                    new_args.push(new_value);
                } else {
                    new_args.push(arg.clone());
                }
            } else if arg.starts_with("!FORMAT:") {
                let split_data = arg.split_once(":");
                if let Some((_, formatable)) = split_data {
                    let mut new_value = formatable.to_string();
                    for (part, value) in changed_version.old.iter() {
                        new_value = new_value.replace(
                            format!("{{old.{}}}", part).as_str(),
                            value.to_string().as_str(),
                        )
                    }
                    for (part, value) in changed_version.new.iter() {
                        new_value = new_value.replace(
                            format!("{{new.{}}}", part).as_str(),
                            value.to_string().as_str(),
                        )
                    }
                    new_args.push(new_value);
                } else {
                    new_args.push(arg.clone());
                }
            } else {
                new_args.push(arg.clone());
            }
        }
        Ok(new_args)
    }

    pub fn execute_afterword_hooks(&self, changed_version: &ChangedVersion) -> CmdResult {
        if let Some(hooks) = &self.profile_model.hooks {
            if let Some(afterwords) = &hooks.afterwords {
                println!("\n \u{1F50D} Founded some afterwords hooks...");
                for (cmd_name, args) in afterwords {
                    println!("=> Executing: {}", cmd_name.cyan());
                    let local_args = self.process_args(&args, &changed_version)?;

                    let executed_command = std::process::Command::new(local_args[0].clone())
                        .args(&local_args[1..])
                        .stdout(std::process::Stdio::piped())
                        .stdout(std::process::Stdio::piped())
                        .output();

                    match executed_command {
                        Err(err) => {
                            return show_err!(
                                [CannotExecuteSubprocess]
                                => "An error occured while executing subproccess",
                                error=err,
                                step=cmd_name
                            )
                        }
                        Ok(cmd) => {
                            let stdout_output = String::from_utf8_lossy(&cmd.stdout);
                            print!("{}", &stdout_output.bright_black());
                            if cmd.stderr.len() > 0 {
                                eprintln!(" \u{1F4A5} Oops! There is an error output too");
                                let stderr_output = String::from_utf8_lossy(&cmd.stderr);
                                return show_err!(
                                    [SubproccessCallFailed]
                                    => "An error occured while executing subproccess",
                                    stderr=stderr_output,
                                    command=cmd_name
                                );
                            }
                        }
                    };
                }
                println!("\n \u{2728} Done executing afterwords scripts!");
            }
        }

        Ok(())
    }
}
