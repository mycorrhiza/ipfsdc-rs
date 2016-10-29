use clap::{ App, SubCommand, Arg, ArgMatches };

use context::Context;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("local")
        .about("List local addresses")
        .args(&[
              Arg::with_name("id")
                .long("id")
                .help("Show peer id in addresses"),
        ])
}

pub fn run(context: &mut Context, matches: &ArgMatches) {
    let addresses = context.event_loop
        .run(context.client.swarm().local_addresses(matches.is_present("id")))
        .expect("TODO: not crash here")
        .addresses;

    for addr in addresses {
        println!("{}", addr);
    }
}
