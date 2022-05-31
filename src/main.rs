use clap::Parser;

mod commands;
mod error;
mod handleable;
mod rtcontext;

fn main() {
    let args = commands::CLI::parse();
    args.handle();
}
