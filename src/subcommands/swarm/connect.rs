use std::str::FromStr;

use clap::{ App, SubCommand, Arg, ArgMatches };
use futures::Future;
use multiaddr::MultiAddr;

use util;
use context::Context;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("connect")
        .about("Open a new connection to given peer(s)")
        .args(&[
            Arg::with_name("address")
                .help("Address(es) of peer(s) to connect to")
                .required(true)
                .takes_value(true)
                .multiple(true)
                .validator(util::multiaddr_validator),
        ])
}

pub fn run(context: &mut Context, matches: &ArgMatches) {
    let Context { ref client, ref mut event_loop, .. } = *context;

    let connections = matches.values_of("address")
        .expect("This argument is required")
        .map(|addr| {
            let addr = MultiAddr::from_str(addr).expect("This is validated");
            client.swarm().connect(&addr)
                .map(move |result| {
                    match result {
                        Ok(msgs) => {
                            for msg in msgs {
                                println!("{}", msg);
                            }
                        }
                        Err(msg) => {
                            println!("{}: {}", addr, msg);
                        }
                    }
                })
        });

    util::run_all(event_loop, connections);
}
