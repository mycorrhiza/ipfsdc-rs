#[macro_use]
extern crate clap;
extern crate tokio_core;
extern crate ipfs_client;

use clap::{App, Arg, SubCommand, AppSettings};

use tokio_core::reactor::Core;
use ipfs_client::Client;

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
            SubCommand::with_name("version")
                .about("Shows the ipfs daemon's version information")
                .setting(AppSettings::DeriveDisplayOrder)
                .args(&[
                    Arg::with_name("number")
                        .short("n")
                        .long("number")
                        .help("Only show the version number"),
                    Arg::with_name("commit")
                        .long("commit")
                        .help("Show the commit hash"),
                    Arg::with_name("repo")
                        .long("repo")
                        .help("Show repo version"),
                ]),
        ])
        .get_matches();

    let mut event_loop = Core::new().unwrap();
    let client = Client::new(event_loop.handle(), "http://localhost:5001/api/v0/");

    match matches.subcommand() {
        ("version", Some(matches)) => {
            let version = event_loop.run(client.version()).expect("TODO: not crash here");
            if matches.is_present("repo") {
                println!("{}", version.repo);
            } else {
                if !matches.is_present("number") {
                    print!("ipfs daemon version ");
                }
                print!("{}", version.version);
                if matches.is_present("commit") {
                    print!("-{}", version.commit);
                }
                println!();
            }
        }
        _ => unreachable!(),
    }
}
