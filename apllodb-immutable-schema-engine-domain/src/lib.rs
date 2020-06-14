// #![deny(warnings, missing_docs, missing_debug_implementations)]

//! Domain layer of apllodb-immutable-schema-engine.

mod entity;
mod row_iter;
mod transaction;
mod version;
mod vtable;

pub use entity::Entity;
pub use row_iter::{ImmutableSchemaRowIter, VersionRowIter};
pub use transaction::ImmutableSchemaTx;
pub use version::{ActiveVersion, InactiveVersion, VersionId, VersionNumber};
pub use vtable::{VTable, VTableId};
