use clap::Parser;

mod commands;
mod handleable;
mod rtcontext;

fn main() {
    let args = commands::CLI::parse();
    args.handle();
}
