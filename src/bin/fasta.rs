#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;

use std::process;


fn validate(_: &clap::ArgMatches) -> Result<(), ()> {
    info!("I haven't a brain with which to think...");
    Ok(())
}

fn main() {
    env_logger::init().unwrap();

    let yaml = load_yaml!("fasta.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let res = match matches.subcommand() {
        ("validate", Some(m)) => validate(m),
        _ => Err(()),
    };

    match res {
        Err(_) => process::exit(1),
        Ok(_) => {},
    }
}
