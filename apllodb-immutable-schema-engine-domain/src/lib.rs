// #![deny(warnings, missing_docs, missing_debug_implementations)]

//! Domain layer of apllodb-immutable-schema-engine.

#[macro_use]
extern crate derive_new;

mod entity;
mod row;
mod row_iter;
mod transaction;
mod version;
mod vtable;

pub use entity::Entity;
pub use row::{ApparentPrimaryKey, FullPrimaryKey, ImmutableRow, ImmutableRowBuilder};
pub use row_iter::{ImmutableSchemaRowIter, VersionRowIter};
pub use transaction::ImmutableSchemaTx;
pub use version::{
    ActiveVersion, ActiveVersions, InactiveVersion, VersionId, VersionNumber, VersionRepository,
};
pub use vtable::{TableWideConstraints, VTable, VTableId, VTableRepository};

pub mod test_support;
