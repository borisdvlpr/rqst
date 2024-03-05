use clap::{command, Arg, ArgMatches};
use std::error::Error;

type ArgResult<T> = Result<T, Box<dyn Error>>;

pub fn parse_args() -> ArgResult<ArgMatches> {
    let match_args = command!()
        .version(env!("CARGO_PKG_VERSION"))
        .about("Rust-based HTTP client for the terminal.")
        .allow_missing_positional(true)
        .arg(
            Arg::new("method")
                .required(false)
                .default_value("GET")
                .help("HTTP Method")
        )
        .arg(
            Arg::new("url")
                .required(true)
                .help("Target url"),
        )
        .get_matches();

    Ok(match_args)
}

pub fn run(match_args: ArgMatches) ->  ArgResult<()> {
    match match_args.get_one::<String>("url") {
        Some(url) => {
            println!("Target url: {}", url);

            Ok(())
        }
        
        None => Ok(()),
    }
}
