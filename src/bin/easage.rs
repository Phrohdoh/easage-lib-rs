extern crate clap;
use clap::{App, AppSettings};

extern crate easage;

mod easage_unpack;
use easage_unpack as unpack;

mod easage_list;
use easage_list as list;

mod easage_pack;
use easage_pack as pack;

const VERSION: &'static str = "0.0.3";

fn main() {
    let matches = App::new("easage")
        .version(VERSION)
        .about("Read, create, and unpack from BIG archives")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(list::get_command())
        .subcommand(pack::get_command())
        .subcommand(unpack::get_command())
        .get_matches();

    let run_result = match matches.subcommand() {
        (list::COMMAND_NAME, Some(args)) => list::run(args),
        (pack::COMMAND_NAME, Some(args)) => pack::run(args),
        (unpack::COMMAND_NAME, Some(args)) => unpack::run(args),
        _ => Ok(()),
    };

    if let Err(err) = run_result {
        eprintln!("ERROR: {}", err);
        std::process::exit(1);
    }
}