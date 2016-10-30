mod peers;
mod addrs;
mod connect;
mod disconnect;

use clap::{ App, AppSettings, SubCommand, ArgMatches };

use context::Context;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("swarm")
        .about("\
            Manipulate the network swarm.\n\
            \n\
            The swarm is the component that opens, listens for, and \
            maintains connections to other ipfs peers in the internet.\
        ")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![
            peers::subcommand(),
            addrs::subcommand(),
            connect::subcommand(),
            disconnect::subcommand(),
        ])
}

pub fn run(context: &mut Context, matches: &ArgMatches) {
    match matches.subcommand() {
        ("peers", Some(matches)) => peers::run(context, matches),
        ("addrs", Some(matches)) => addrs::run(context, matches),
        ("connect", Some(matches)) => connect::run(context, matches),
        ("disconnect", Some(matches)) => disconnect::run(context, matches),
        _ => unreachable!(),
    }
}
