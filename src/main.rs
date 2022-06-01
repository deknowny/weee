use clap::Parser;

mod commands;
mod config;
mod context;
mod error;
mod handleable;
mod tests;

fn main() {
    let args = commands::CLI::parse();
    args.handle();
}
