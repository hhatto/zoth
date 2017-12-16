use clap::{Arg, App, SubCommand};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn build_zothcl() -> App<'static, 'static> {
    App::new("zothcl")
        .version(VERSION)
        .about("CLI for zothd")
        .subcommand(
            SubCommand::with_name("exec")
            .about("execution command with using cache")
            .arg(Arg::with_name("cmd").multiple(true).last(true)))
        .subcommand(
            SubCommand::with_name("clear")
            .about("clear all cache data")
            .arg(Arg::with_name("cmd").multiple(true).last(true)))
        .subcommand(
            SubCommand::with_name("shutdown")
            .about("clear all cache data"))
}
