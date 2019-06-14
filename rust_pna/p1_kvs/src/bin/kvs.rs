extern crate clap;

use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
    let matches = App::new("kvs")
        .version("0.1.0")
        .author("Pedro H. Dias <pdcmpa@gmail.com>")
        .about("Key / Value Store")
        .subcommand(
            SubCommand::with_name("get").about("kvs get <KEY>").arg(
                Arg::with_name("key")
                    .help("The key to search for")
                    .index(1)
                    .required(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("kvs set <KEY> <VALUE>")
                .args(&[
                    Arg::with_name("key")
                        .help("The key to override")
                        .index(1)
                        .required(true),
                    Arg::with_name("value")
                        .help("The value to use")
                        .index(2)
                        .required(true),
                ]),
        )
        .subcommand(
            SubCommand::with_name("rm").about("kvs rm <KEY>").arg(
                Arg::with_name("key")
                    .help("The key to remove")
                    .index(1)
                    .required(true),
            ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("get") => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        Some("set") => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        Some("rm") => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        None => process::exit(1),
        _ => println!("Some other subcommand was used"),
    }
}
