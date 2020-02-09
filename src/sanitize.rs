use crate::prelude::*;
use regex::Regex;

fn name(s: &str) -> Result<(), GenericNameError> {
    lazy_static! {
        static ref GOOD_NAME: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
        static ref NO_NUMBER_BEGIN: Regex = Regex::new(r"^[_a-zA-Z]").unwrap();
        static ref HAS_ALPHANUMERIC: Regex = Regex::new(r"[a-zA-Z0-9]").unwrap();
    }

    if s.len() > 40 {
        return Err(GenericNameError::TooLong(s.to_string()));
    }

    if s.is_empty() {
        return Err(GenericNameError::TooShort(s.to_string()));
    }

    if !GOOD_NAME.is_match(s) {
        return Err(GenericNameError::BadInternalChars(s.to_string()))
    }

    if !NO_NUMBER_BEGIN.is_match(s) {
        return Err(GenericNameError::BeginNumber(s.to_string()))
    }

    if !HAS_ALPHANUMERIC.is_match(s) {
        return Err(GenericNameError::JustUnderscores(s.to_string()))
    }

    if s.ends_with('_') {
        return Err(GenericNameError::EndUnderscore(s.to_string()))
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum GenericNameError {
    #[error("'{0}' is too long (max 40 chars)")]
    TooLong(String),

    #[error("'{0}' is too short (min 1 char)")]
    TooShort(String),

    #[error("'{0}' has non-alphanumeric/underscore chars")]
    BadInternalChars(String),

    #[error("'{0}' is just underscores")]
    JustUnderscores(String),

    #[error("'{0}' begins with a number")]
    BeginNumber(String),

    #[error("'{0}' ends in an underscore")]
    EndUnderscore(String),
}

pub fn package_name(s: &str) -> Result<(), PackageNameError> {
    lazy_static! {
        static ref RESERVED_PACKAGE_NAMES: Vec<&'static str> = vec![ "pm64" ];
    }

    if let Some('_') = s.chars().nth(0) {
        return Err(PackageNameError::BeginsUnderscore(s.to_string()));
    }

    if RESERVED_PACKAGE_NAMES.iter().any(|&a| a == s) {
        return Err(PackageNameError::Reserved(s.to_string()));
    }

    name(s).map_err(Into::into)
}

#[derive(Error, Debug)]
pub enum PackageNameError {
    #[error("invalid package name: '{0}' is reserved")]
    Reserved(String),

    #[error("invalid package name: '{0}' begins with an underscore")]
    BeginsUnderscore(String),

    #[error("invalid package name: {0}")]
    Generic(#[from] GenericNameError)
}

pub fn export_name(s: &str) -> Result<(), ExportNameError> {
    lazy_static! {
        static ref UPPER_ALPHA_BEGIN: Regex = Regex::new(r"^[A-Z]").unwrap();
    }

    if UPPER_ALPHA_BEGIN.is_match(s) {
        warn!("export name '{}' begins with an uppercase letter", s);
    }

    name(s).map_err(Into::into)
}

#[derive(Error, Debug)]
pub enum ExportNameError {
    #[error("invalid export name: {0}")]
    Generic(#[from] GenericNameError)
}

pub fn dependency_name(s: &str, current_pkg_name: &str) -> Result<(), DependencyNameError> {
    if s == current_pkg_name {
        return Err(DependencyNameError::SameAsCurrentPackage(s.to_string()))
    }

    name(s).map_err(Into::into)
}

#[derive(Error, Debug)]
pub enum DependencyNameError {
    #[error("invalid dependency name: '{0}' is the same as the current package name")]
    SameAsCurrentPackage(String),

    #[error("invalid dependency name: {0}")]
    Generic(#[from] GenericNameError)
}
