use crate::prelude::*;
use super::Package;
use std::collections::HashMap;

/// A fully-qualified identifier.
#[derive(Hash, Clone, Eq, PartialEq)]
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
        Identifier::new(&pkg.manifest.name, name)
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.package, self.name)
    }
}

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", &self.package, &self.name)
    }
}

pub trait Identify : std::hash::Hash + Sized + Eq {
    type T;

    fn new(pkg_name: &str, t_name: &str) -> Self;

    fn identify(pkg: &Package, t: &Self::T) -> Self;

    fn resolve<'a>(&self, map: &'a HashMap<Self, Self::T>) -> Option<&'a Self::T> {
        map.get(self)
    }

    /// Parses the given string from the format: `package/name`. If the package substring is
    /// missing, it is assumed to be the name of the source package.
    fn parse(string: &str, source_pkg_name: &str) -> Result<Self, ParseError> {
        if string.is_empty() {
            return Err(ParseError::Empty);
        }

        Ok(if string.contains('/') {
            let mut it = string.splitn(1, '/');

            Self::new(it.next().unwrap(), match it.next() {
                Some(pkg_name) => Ok(pkg_name),
                None => Err(ParseError::NameRequired),
            }?)
        } else {
            Self::new(source_pkg_name, string)
        })
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("identifier is empty")]
    Empty,

    #[error("export name required after the '/'")]
    NameRequired,
}
