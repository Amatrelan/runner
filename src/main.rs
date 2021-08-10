use std::fs;
use std::path::Path;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};
use log::info;
mod config;
mod logging;

fn parse_args() -> ArgMatches {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::new("log_level")
                .short('l')
                .long("log-level")
                .takes_value(true)
                .value_name("LOG_LEVEL")
                .possible_values(&["trace", "debug", "info", "error", "warn"])
                .default_value("info")
                .about("Set log level for this"),
        )
        .arg(
            Arg::new("config-path")
                .short('c')
                .long("config")
                .takes_value(true)
                .value_name("PATH")
                .about("Config path to run"),
        )
        .subcommand(
            App::new("generate")
                .version(crate_version!())
                .about("Automatically generate template configs.")
                .arg(
                    Arg::new("gen-config-path")
                        .about("Generate new config file.")
                        .short('p')
                        .long("path")
                        .takes_value(true)
                        .value_name("PATH")
                        .about("Path where generate new config"),
                ),
        )
        .get_matches()
}

fn main() {
    let args: ArgMatches = parse_args();
    // let opts: Opts = Opts::parse();
    logging::setup_logging(args.value_of("log_level").unwrap());

    // if opts.generate_config.unwrap() {
    //     if Path::new(&opts.file_path).exists() {
    //         panic!("File already exists.")
    //     }

    //     let config = config::generate_default_config(opts.file_path.clone());
    // }

    // let config_content = fs::read_to_string(opts.file_path).unwrap();

    // config::deserialize_config(config_content);
}
