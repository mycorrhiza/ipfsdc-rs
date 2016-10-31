use std::fmt::Debug;
use std::str::FromStr;

use base58::FromBase58;
use futures::{ self, Future };
use multihash::{ MultiHash, ReadMultiHash };
use multiaddr::MultiAddr;
use tokio_core::reactor::Core;

pub fn run_all<F, I>(event_loop: &mut Core, futures: I)
    where F: Future<Item=()>,
          I: IntoIterator<Item=F>,
          <F as Future>::Error: Debug
{
    let mut futures: Vec<_> = futures.into_iter().collect();
    while !futures.is_empty() {
        futures = match event_loop.run(futures::select_all(futures)) {
            Ok((_, _, remaining)) => remaining,
            Err((err, _, remaining)) => {
                println!("TODO handle err: {:?}", err);
                remaining
            }
        };
    }
}

pub fn multiaddr_validator(s: String) -> Result<(), String> {
    MultiAddr::from_str(&*s).map(|_| ()).map_err(|e|
        format!("Failed to parse multiaddr {:?}: {}", s, e))
}

pub fn parse_multihash(s: &str) -> Result<MultiHash, String> {
    s.from_base58()
        .and_then(|b| (&b[..]).read_multihash().map_err(|e| format!("{}", e)))
        .map_err(|e| format!("Failed to parse multihash {:?}: {}", s, e))
}

pub fn multihash_validator(s: String) -> Result<(), String> {
    parse_multihash(&*s).map(|_| ())
}
