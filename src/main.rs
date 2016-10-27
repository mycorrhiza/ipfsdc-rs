#[macro_use]
extern crate clap;
extern crate futures;
extern crate ipfs_client;
extern crate tokio_core;
extern crate multiaddr;

use clap::{ App, AppSettings, Arg };

use context::Context;

mod context;

mod info;
mod version;

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
        .subcommands(vec![
            info::subcommand(),
            version::subcommand(),
        ])
        .args(&[
            Arg::with_name("api")
                .long("api")
                .help("Specify the ipfs daemon to connect to")
                .default_value("/ip4/127.0.0.1/tcp/5001/https")
        ])
        .get_matches();

    let mut context = Context::new(&matches);

    match matches.subcommand() {
        ("info", Some(matches)) => info::run(&mut context, matches),
        ("version", Some(matches)) => version::run(&mut context, matches),
        _ => unreachable!(),
    }
}
