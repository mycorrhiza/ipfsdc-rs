use std::cmp;

use clap::{ App, Arg, SubCommand, ArgMatches };
use ipfs_client::data::PeerInfo;

use context::Context;
use util;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("info")
        .visible_alias("id")
        .about("Show ipfs peer info.")
        .after_help("\
            Prints out information about the specified peer(s).\n\
            If no peers are specified prints out information about the local peer.\
        ")
        .args(&[
            Arg::with_name("peerid")
                .takes_value(true)
                .help("Id of peer to lookup")
                .validator(util::multihash_validator),
        ])
}

pub fn run(context: &mut Context, matches: &ArgMatches) {
    let future = matches.value_of("peerid")
        .map(|s| util::parse_multihash(s).expect("impossible: validated in arg"))
        .map(|id| context.client.peer_info(&id))
        .unwrap_or_else(|| context.client.local_info());

    print(context.event_loop.run(future).expect("TODO: not crash here"));
}

fn print(info: PeerInfo) {
    println!("peer id:");
    println!("    {}", info.id);
    println!();
    println!("public key:");
    let mut rest = &*info.public_key;
    while !rest.is_empty() {
        let (line, next) = rest.split_at(cmp::min(100, rest.len()));
        println!("    {}", line);
        rest = next;
    }
    println!();
    println!("addresses:");
    for addr in info.addresses {
        println!("    {}", addr);
    }
    println!();
    println!("agent version:");
    println!("    {}", info.agent_version);
    println!();
    println!("protocol version:");
    println!("    {}", info.protocol_version);
}

