extern crate base58;
#[macro_use]
extern crate clap;
extern crate futures;
extern crate ipfs_client;
extern crate tokio_core;
extern crate multihash;
extern crate multiaddr;

mod util;
mod context;
mod subcommands;

use clap::{ App, AppSettings };

use context::Context;

fn main() {
    let matches = App::new("IPFS Daemon CLI")
        .author(crate_authors!())
        .version(crate_version!())
        .settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::VersionlessSubcommands,
        ])
        .global_settings(&[
            AppSettings::ColoredHelp,
            AppSettings::DeriveDisplayOrder,
        ])
        .subcommands(subcommands::subcommands())
        .args(&*Context::args())
        .get_matches();

    subcommands::run(&mut Context::new(&matches), matches);
}
