#![allow(unknown_lints)] // for clippy
#![warn(fat_ptr_transmutes)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
// TODO #![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_import_braces)]
#![warn(unused_results)]
#![warn(variant_size_differences)]

#[macro_use]
extern crate clap;
extern crate futures;
extern crate ipfs_client;
extern crate tokio_core;
extern crate mhash;
extern crate maddr;

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
