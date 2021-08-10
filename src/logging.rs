use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

pub(crate) fn setup_logging(opts: &str) {
    let level = match opts.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        _ => panic!("No valid logging level"),
    };

    println!("Log level set to: {:?}", opts);

    let _ = CombinedLogger::init(vec![TermLogger::new(
        level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )]);
}
