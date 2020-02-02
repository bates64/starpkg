use crate::prelude::*;
use log::{Record, Level, LevelFilter, Metadata, SetLoggerError};

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // Print file:line information in debug mode
            #[cfg(debug_assertions)] {
                if let Some(file) = record.file() {
                    if let Some(line) = record.line() {
                        eprint!("{} ", Color::Fixed(8).normal().paint(
                            format!("{:>16}:{:<3}", &file[4..], line)
                        ));
                    }
                }
            }

            eprintln!("{:>5} {}", match record.level() {
                Level::Error => Color::Fixed(9).normal().paint("error:"),
                Level::Warn  => Color::Fixed(3).normal().paint("warn:"),
                Level::Info  => Color::Fixed(10).normal().paint("info:"),
                Level::Debug => Color::Fixed(14).normal().paint("debug:"),
                Level::Trace => Color::Fixed(8).normal().paint("trace:"),
            }, record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

pub fn init(verbosity: usize) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(match verbosity {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }))
}
