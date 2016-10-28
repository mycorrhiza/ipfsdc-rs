use clap::{ App, SubCommand, ArgMatches };

use context::Context;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("peers")
        .about("List the set of peers this node is connected to")
}

pub fn run(context: &mut Context, _: &ArgMatches) {
    let peers = context.event_loop
        .run(context.client.swarm().peers())
        .expect("TODO: not crash here");

    for addr in peers.addresses {
        println!("{}", addr);
    }
}
