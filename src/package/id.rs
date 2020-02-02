use crate::prelude::*;
use super::Package;

/// A fully-qualified identifier.
#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub struct Identifier {
    package: String,
    name: String,
}

impl Identifier {
    // TODO: disallow bad characters in either name

    pub fn new(pkg_name: &str, name: &str) -> Identifier {
        Identifier {
            package: pkg_name.to_owned(),
            name: name.to_owned(),
        }
    }

    pub fn from_package(pkg: &Package, name: &str) -> Identifier {
        Identifier::new(&format!("{}", pkg.manifest.name), name)
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.package, self.name)
    }
}

pub trait Identify : std::hash::Hash {
    type T;

    fn new(pkg_name: &str, t_name: &str) -> Self;

    fn identify(pkg: &Package, t: &Self::T) -> Self;

    fn resolve<'pkg>(&self, pkg: &'pkg Package) -> Option<&'pkg Self::T>;
}
