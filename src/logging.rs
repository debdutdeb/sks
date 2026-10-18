// src/logging.rs
use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::path::Path;

pub fn init(log_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let pattern = "{d(%Y-%m-%d %H:%M:%S)} {l} {t} - {m}{n}";

    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(log_file)?;

    let config = Config::builder()
        .appender(Appender::builder().build("stderr", Box::new(stderr)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("stderr")
                .appender("file")
                .build(LevelFilter::Info),
        )?;

    log4rs::init_config(config)?;
    Ok(())
}
