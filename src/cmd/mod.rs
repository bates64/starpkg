pub mod new;
pub mod build;

use crate::prelude::*;
use crate::package::Package;

#[derive(Debug)]
pub struct CommandContext {
    pub package: Result<Package, TaggedError>,
}

/// If the package could not be read, we also remember the path it was supposed to be at.
/// This is used by the `new` command to know where to place the new package.
#[derive(Error, Debug)]
#[error("{source}")]
pub struct TaggedError {
    dir: PathBuf,
    #[source]
    source: Error,
}

impl CommandContext {
    pub fn new(package_dir: Option<PathBuf>) -> CommandContext {
        let current_dir = std::env::current_dir().unwrap();

        CommandContext {
            package: match package_dir {
                Some(path) => Package::from_dir(&path)
                    .map_err(|err| TaggedError {
                        dir: path.to_owned(),
                        source: err.into(),
                    }),
                None => Package::find(&current_dir)
                    .map_err(|err| TaggedError {
                        dir: current_dir,
                        source: err.into(),
                    }),
            }
        }
    }

    pub fn package_dir(&self) -> &Path {
        match &self.package {
            Ok(package)                   => &package.dir,
            Err(TaggedError { dir, .. }) => &dir,
        }
    }
}
