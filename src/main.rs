

use clap::Parser;





mod commands;
mod error;
mod handleable;
mod context;
mod tests;
mod config;


fn main() {
    let args = commands::CLI::parse();
    args.handle();
}
