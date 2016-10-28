mod info;
mod version;
mod swarm;

use clap::{ App, ArgMatches };

use context::Context;

pub fn subcommands() -> Vec<App<'static, 'static>> {
    vec![
        info::subcommand(),
        version::subcommand(),
        swarm::subcommand(),
    ]
}

pub fn run(context: &mut Context, matches: ArgMatches) {
    match matches.subcommand() {
        ("info", Some(matches)) => info::run(context, matches),
        ("version", Some(matches)) => version::run(context, matches),
        ("swarm", Some(matches)) => swarm::run(context, matches),
        _ => unreachable!(),
    }
}
