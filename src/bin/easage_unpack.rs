use ::std::fs::{self, OpenOptions};
use ::std::io::Write;
use ::std::path::PathBuf;
use clap::{Arg, ArgMatches, App, SubCommand};

use ::easage::{Archive, LibResult};

pub const COMMAND_NAME: &'static str = "unpack";
const ARG_NAME_SOURCE: &'static str = "source";
const ARG_NAME_OUTPUT: &'static str = "output";

const VERSION: &'static str = "0.0.1";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME)
        .version(VERSION)
        .about("Unpack the contents of a BIG archive")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .arg(Arg::with_name(ARG_NAME_SOURCE)
                .long(ARG_NAME_SOURCE)
                .value_name(ARG_NAME_SOURCE)
                .takes_value(true)
                .required(true)
                .help("Path to the BIG to unpack."))
        .arg(Arg::with_name(ARG_NAME_OUTPUT)
                .long(ARG_NAME_OUTPUT)
                .value_name(ARG_NAME_OUTPUT)
                .takes_value(true)
                .required(true)
                .help("Path to the directory that should contain the BIG archive's contents."))
}

pub fn run(args: &ArgMatches) -> LibResult<()> {
    let source = args.value_of(ARG_NAME_SOURCE).unwrap();
    let output = args.value_of(ARG_NAME_OUTPUT).unwrap();
    let output = PathBuf::from(output);

    let mut archive = Archive::from_path(source)?;
    let table = archive.read_entry_metadata_table()?;

    for (entry_name, _entry) in &table {
        if let Some(data) = archive.get_bytes_via_table(&table, entry_name) {
            let output_file = {
                let mut o = output.clone();
                o.push(entry_name.replace("\\", "/"));
                o
            };

            // TODO: We need our own error type for this case.
            let output_dir = output_file.parent().unwrap();

            let _ = fs::create_dir_all(&output_dir);

            let mut f = OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .truncate(true)
                .open(&output_file)?;

            f.write(data)?;
        }
    }

    Ok(())
}