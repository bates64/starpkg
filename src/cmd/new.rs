use crate::prelude::*;
use structopt::StructOpt;
use super::Context;

use crate::package::Package;

#[derive(StructOpt, Debug)]
pub struct Opt {
    /// The name of the package to generate
    name: String,
}

pub fn run(ctx: Context, opt: Opt) -> Result<()> {
    let package = Package::new(ctx.package_dir(), opt.name)?;

    info!("created package {} at {}", &package, package.dir.display());

    Ok(())
}
