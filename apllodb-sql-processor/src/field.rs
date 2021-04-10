//! A field is a part of a record.
//! Types of fields are:
//!
//! - attribute in correlation (denoted as `correlation.attribute`).
//! - constant (e.g. `777`, `"abc").

pub(crate) mod aliased_field_name;
pub(crate) mod field_alias;
pub(crate) mod field_name;
