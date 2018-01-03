use ::std::path::PathBuf;
use clap::{Arg, ArgMatches, App, SubCommand};

use ::easage::{self, Kind, LibResult};

pub const COMMAND_NAME: &'static str = "pack";
const ARG_NAME_SOURCE: &'static str = "source";
const ARG_NAME_OUTPUT: &'static str = "output";
const ARG_NAME_KIND: &'static str = "kind";
const ARG_NAME_STRIP_PREFIX: &'static str = "strip-prefix";

const VERSION: &'static str = "0.0.3";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME)
        .version(VERSION)
        .about("Recursively package a directory structure into a BIG archive")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .arg(Arg::with_name(ARG_NAME_SOURCE)
                .long(ARG_NAME_SOURCE)
                .value_name(ARG_NAME_SOURCE)
                .takes_value(true)
                .required(true)
                .help("Path to the directory to pack into a BIG archive."))
        .arg(Arg::with_name(ARG_NAME_OUTPUT)
                .long(ARG_NAME_OUTPUT)
                .value_name(ARG_NAME_OUTPUT)
                .takes_value(true)
                .required(true)
                .help("Path to the output BIG archive."))
        .arg(Arg::with_name(ARG_NAME_KIND)
                .long(ARG_NAME_KIND)
                .value_name(ARG_NAME_KIND)
                .takes_value(true)
                .required(true)
                .validator(validate_is_bigf_or_big4)
                .help("BIG archive kind (BIGF or BIG4, case-sensitive)"))
        .arg(Arg::with_name(ARG_NAME_STRIP_PREFIX)
                .long(ARG_NAME_STRIP_PREFIX)
                .value_name(ARG_NAME_STRIP_PREFIX)
                .takes_value(true)
                .help("A path prefix to strip from entry names"))
}

pub fn run(args: &ArgMatches) -> LibResult<()> {
    let source = args.value_of(ARG_NAME_SOURCE).unwrap();

    let output = args.value_of(ARG_NAME_OUTPUT).unwrap();
    let output = PathBuf::from(output);

    let kind = args.value_of(ARG_NAME_KIND).unwrap();
    let kind = Kind::from_bytes(kind.as_bytes());

    let strip_prefix = args.value_of(ARG_NAME_STRIP_PREFIX);

    easage::pack_directory(&source, &output, kind, strip_prefix)
}

fn validate_is_bigf_or_big4(v: String) -> Result<(), String> {
    if v != "BIG4" && v != "BIGF" {
        Err(format!("{} must be one of 'BIGF' or 'BIG4'", ARG_NAME_KIND))
    } else {
        Ok(())
    }
}