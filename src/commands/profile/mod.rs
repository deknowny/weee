use clap;

mod add;
mod remove;


#[derive(Debug, clap::Subcommand)]
pub enum ProfileCommands {
    Add(add::Add)
}


#[derive(Debug, clap::Args)]
pub struct Profile {
    #[clap(subcommand)]
    command: ProfileCommands
}
