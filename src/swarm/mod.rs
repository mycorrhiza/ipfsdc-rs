mod peers;

use clap::{ App, SubCommand, ArgMatches };

use context::Context;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("swarm")
        .about("\
            Manipulate the network swarm.\n\
            \n\
            The swarm is the component that opens, listens for, and \
            maintains connections to other ipfs peers in the internet.\
        ")
        .subcommands(vec![
            peers::subcommand(),
        ])
}

pub fn run(context: &mut Context, matches: &ArgMatches) {
    match matches.subcommand() {
        ("peers", Some(matches)) => peers::run(context, matches),
        _ => unreachable!(),
    }
}
