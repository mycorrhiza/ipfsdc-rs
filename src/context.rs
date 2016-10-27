use std::marker::PhantomData;

use clap::ArgMatches;
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
}
