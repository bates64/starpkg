use crate::prelude::*;
use crate::starrod::StarRod;
use structopt::StructOpt;
use super::CommandContext;

use std::time::Instant;

#[derive(StructOpt, Debug)]
pub struct Opt {
    /// Skip compilation via Star Rod.
    #[structopt(long)]
    no_compile: bool,
}

pub fn run(ctx: CommandContext, opt: Opt) -> Result<()> {
    let mut package = ctx.package?;

    trace!("assembling package:\n{:#?}", &package);

    let build_dir = package.dir.join(".build");
    if !build_dir.is_dir() {
        debug!("creating build directory");
        fs::create_dir(&build_dir)
            .with_context(|| "unable to create build directory")?;
    }

    let start_time = Instant::now();
    package.assemble(&build_dir)?;
    info!("assembled {} in {}s", &package, start_time.elapsed().as_secs_f32());

    if opt.no_compile {
        debug!("skipping compilation due to --no-compile flag");
    } else {
        let sr = StarRod::new_or_download()?;
        trace!("{:?}", sr);

        // TODO
    }

    Ok(())
}
