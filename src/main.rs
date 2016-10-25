#[macro_use]
extern crate clap;
extern crate tokio_core;
extern crate ipfs_client;

use clap::{ App, AppSettings };

use context::Context;

mod context;
mod version;

fn main() {
    let matches = App::new("IPFS Daemon Client")
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
        .subcommands(vec![
            version::subcommand(),
        ])
        .get_matches();

    let mut context = Context::new();

    match matches.subcommand() {
        ("version", Some(matches)) => version::run(&mut context, matches),
        _ => unreachable!(),
    }
}
