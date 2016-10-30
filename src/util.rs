use std::fmt::Debug;
use std::str::FromStr;

use futures::{ self, Future };
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
        format!("Failed to parse multiaddr {}: {}", s, e))
}
