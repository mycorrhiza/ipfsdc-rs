mod local;

use clap::{ App, SubCommand, ArgMatches };

use context::Context;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("addrs")
        .about("List known addresses")
        .subcommands(vec![
            local::subcommand(),
        ])
}

pub fn run(context: &mut Context, matches: &ArgMatches) {
    match matches.subcommand() {
        ("local", Some(matches)) => local::run(context, matches),
        (_, None) => run_self(context, matches),
        _ => unreachable!(),
    }
}

fn run_self(context: &mut Context, _: &ArgMatches) {
    let peers = context.event_loop
        .run(context.client.swarm().addresses())
        .expect("TODO: not crash here")
        .peers;

    for (peer, addrs) in peers {
        println!("{} ({}):", peer, addrs.len());
        for addr in addrs {
            println!("        {}", addr);
        }
    }
}
