mod prelude {
    pub use std::{io, fs, fmt};
    pub use std::path::{PathBuf, Path};
    pub use std::fmt::Write;

    pub use lazy_static::lazy_static;

    // Logging
    pub use log::{error, warn, info, debug, trace};
    pub use ansi_term::{ANSIString, Color};

    // Errors
    pub use thiserror::Error;
    pub use anyhow::{Context, Result, Error, anyhow};

    // Serde
    pub use serde::{Serialize, Deserialize, de, ser};
    pub use semver::{Version, VersionReq};

    #[cfg(test)]
    pub use pretty_assertions::{assert_eq, assert_ne};
}

mod logger;
mod cmd;
mod package;
mod sanitize;

use prelude::*;
use std::path::PathBuf;
use structopt::{StructOpt, clap::AppSettings::*};

/// A tool for creating composable Paper Mario mods.
#[derive(StructOpt, Debug)]
#[structopt(name = "starpkg", global_setting(ColorAuto), global_setting(ColoredHelp))]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,

    /// Path to package directory
    #[structopt(short = "d", long = "dir", parse(from_os_str))]
    package: Option<PathBuf>,

    /// Verbosity level (-v: debug, -vv: trace)
    #[structopt(short = "v", parse(from_occurrences))]
    verbosity: usize,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Sets up a new package
    New(cmd::new::Opt),

    /// Assembles dependencies
    // TODO: also compile mod
    Build(cmd::build::Opt),
}

fn main() {
    if let Err(err) = try_main() {
        let mut chain = err.chain();

        error!("{}", chain.next().unwrap());

        for cause in chain {
            trace!("{}", Color::Fixed(8).normal().paint(format!("{}", cause)));
        }

        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let opt = Opt::from_args();

    logger::init(opt.verbosity)?;
    if opt.verbosity > 2 {
        warn!("superfluous verbosity (-vv is max)");
    }

    let ctx = cmd::CommandContext::new(opt.package);

    match opt.cmd {
        Command::New(cmd_opt) => cmd::new::run(ctx, cmd_opt),
        Command::Build(cmd_opt) => cmd::build::run(ctx, cmd_opt),
    }
}
