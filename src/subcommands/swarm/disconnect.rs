use std::str::FromStr;

use clap::{ App, SubCommand, Arg, ArgMatches };
use futures::Future;
use maddr::MultiAddr;

use util;
use context::Context;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("disconnect")
        .about("\
               Close connection(s) to the given address(es)\
               \n\
               \n\
               The disconnect is not permanent; if ipfs needs to talk \
               to that address later, it will reconnect.")
        .args(&[
            Arg::with_name("address")
                .help("Address(es) of peer(s) to disconnect from")
                .required(true)
                .takes_value(true)
                .multiple(true)
                .validator(util::multiaddr_validator),
        ])
}

pub fn run(context: &mut Context, matches: &ArgMatches) {
    let Context { ref client, ref mut event_loop, .. } = *context;

    let disconnections = matches.values_of("address")
        .expect("This argument is required")
        .map(|addr| {
            let addr = MultiAddr::from_str(addr).expect("This is validated");
            client.swarm().disconnect(&addr)
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

    util::run_all(event_loop, disconnections);
}
