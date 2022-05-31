use clap;

mod bump;
mod init;
mod profile;

use crate::handleable::{CmdResult, Handleable};
use crate::context::RTContext;

#[derive(Debug, clap::Subcommand)]
enum Commands {
    Bump(bump::Bump),
    Profile(profile::Profile),
    Init(init::Init),
}

impl Handleable for Commands {
    fn handle(&self, ctx: &mut RTContext) -> CmdResult {
        match self {
            Self::Bump(inst) => inst.handle(ctx),
            Self::Init(inst) => inst.handle(ctx),
            Self::Profile(inst) => inst.handle(ctx),
        }
    }
}

#[derive(Debug, clap::Parser)]
#[clap(
    name = "weee",
    author, version, about, long_about = None
)]
pub struct CLI {
    #[clap(subcommand)]
    command: Commands,
}

impl CLI {
    pub fn handle(self) {
        let mut context = RTContext::new();
        let result = self.command.handle(&mut context);
        dbg!(&result);
    }
}
