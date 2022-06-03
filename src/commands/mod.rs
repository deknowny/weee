use clap;
use colored::Colorize;
use terminal_size::{terminal_size, Height, Width};

mod bump;
mod init;
mod profile;
mod r#move;

use crate::context::RTContext;
use crate::handleable::{CmdResult, Handleable};

#[derive(Debug, clap::Subcommand)]
enum Commands {
    Bump(bump::Bump),
    Profile(profile::Profile),
    Init(init::Init),
    Move(r#move::Move)
}

impl Handleable for Commands {
    fn handle(&self, ctx: &mut RTContext) -> CmdResult {
        match self {
            Self::Bump(inst) => inst.handle(ctx),
            Self::Init(inst) => inst.handle(ctx),
            Self::Profile(inst) => inst.handle(ctx),
            Self::Move(inst) => inst.handle(ctx),
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

        if let Err(err) = result {
            let term_size = terminal_size();
            let header;
            if let Some((Width(w), Height(_))) = term_size {
                let header_block = format!(" [ {} ] ", err.title.red());
                let line = std::iter::repeat("-")
                    .take((w as usize - err.title.len() - 6) / 2)
                    .collect::<String>();
                header = format!("{}{}{}", line, header_block, line)
            } else {
                header = format!("[ {} ]", err.title.red())
            }

            let mut payload = String::new();
            for (key, value) in err.payload.iter() {
                payload.push_str(format!("\n [{}]: {}", key.magenta(), value).as_str());
            }

            eprintln!("{}\n => {}\n{}", header, err.description.yellow(), payload);
        }
    }
}
