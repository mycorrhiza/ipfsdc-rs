use std::marker::PhantomData;

use clap::{ Arg, ArgMatches };
use ipfs_client::Client;
use tokio_core::reactor::Core;

pub struct Context {
    pub event_loop: Core,
    pub client: Client,
    private_construction: PhantomData<bool>,
}

impl Context {
    pub fn new(matches: &ArgMatches) -> Context {
        let event_loop = Core::new().expect("TODO: what could go wrong here?");
        let host = matches.value_of("api").unwrap().parse().expect("TODO: Report parse errors nicer");
        let client = Client::new(event_loop.handle(), host);
        Context {
            event_loop: event_loop,
            client: client,
            private_construction: PhantomData,
        }
    }

    pub fn args() -> Vec<Arg<'static, 'static>> {
        vec![
            Arg::with_name("api")
                .long("api")
                .help("Specify the ipfs daemon to connect to")
                .default_value("/ip4/127.0.0.1/tcp/5001/https")
        ]
    }
}
