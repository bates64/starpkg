use crate::prelude::*;
use structopt::StructOpt;
use super::CommandContext;

use std::time::Instant;

#[derive(StructOpt, Debug)]
pub struct Opt {}

pub fn run(ctx: CommandContext, _: Opt) -> Result<()> {
    let package = ctx.package?;

    debug!("building package:\n{:#?}", &package);

    let build_dir = package.dir.join(".build");
    if !build_dir.is_dir() {
        debug!("creating build directory");
        fs::create_dir(&build_dir)
            .with_context(|| "unable to create build directory")?;
    }

    let start_time = Instant::now();
    package.assemble(&build_dir)?;
    info!("assembled {} in {}s", &package, start_time.elapsed().as_secs_f32());

    // TODO: compile with star rod

    Ok(())
}
