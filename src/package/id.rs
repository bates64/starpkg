use crate::prelude::*;
use crate::sanitize;
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
            let split: Vec<&str> = string.splitn(2, '/').collect();

            let pkg_name = split[0];
            let export_name = match split.get(1) {
                Some(export_name) => export_name,
                None => return Err(ParseError::NameRequired(string.to_string())),
            };

            sanitize::dependency_name(pkg_name, source_pkg_name)
                .map_err(|error| match error {
                    sanitize::DependencyNameError::SameAsCurrentPackage { .. } => {
                        ParseError::PackageNameUnneeded(string.to_string())
                    },
                    _ => ParseError::DependencyNameError(error),
                })?;

            sanitize::export_name(export_name)?;

            if export_name.starts_with('_') {
                return Err(ParseError::Private((*export_name).to_string()));
            }

            Self::new(pkg_name, export_name)
        } else {
            sanitize::export_name(string)?;
            Self::new(source_pkg_name, string)
        })
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("identifier cannot be empty")]
    Empty,

    #[error("identifier '{0}' requires an export name after the slash")]
    NameRequired(String),

    #[error("identifier '{0}' references a private export")]
    Private(String),

    #[error("identifier '{0}' needlessly references the current package explicitly")]
    PackageNameUnneeded(String),

    #[error(transparent)]
    DependencyNameError(#[from] sanitize::DependencyNameError),

    #[error(transparent)]
    ExportNameError(#[from] sanitize::ExportNameError),
}
