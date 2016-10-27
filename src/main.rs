#[macro_use]
extern crate clap;
extern crate futures;
extern crate ipfs_client;
extern crate tokio_core;

use clap::{ App, AppSettings };

use context::Context;

mod context;

mod info;
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
            info::subcommand(),
            version::subcommand(),
        ])
        .get_matches();

    let mut context = Context::new();

    match matches.subcommand() {
        ("info", Some(matches)) => info::run(&mut context, matches),
        ("version", Some(matches)) => version::run(&mut context, matches),
        _ => unreachable!(),
    }
}
