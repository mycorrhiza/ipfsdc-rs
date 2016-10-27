use clap::{ App, Arg, SubCommand, ArgMatches };

use context::Context;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("version")
        .about("Shows the ipfs daemon's version information")
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
        ])
}

pub fn run(context: &mut Context, matches: &ArgMatches) {
    let version = context.event_loop
        .run(context.client.version())
        .expect("TODO: not crash here");

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
