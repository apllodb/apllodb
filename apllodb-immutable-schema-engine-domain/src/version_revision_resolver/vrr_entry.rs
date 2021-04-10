use std::{fmt::Debug, hash::Hash};

use apllodb_shared_components::{ApllodbResult, SqlValue};

use crate::{
    abstract_types::ImmutableSchemaAbstractTypes, entity::Entity,
    row::pk::apparent_pk::ApparentPrimaryKey, row::pk::full_pk::revision::Revision,
    version::id::VersionId,
};

#[derive(PartialEq, Hash, Debug, new)] // Clone here doesn't work. `Engine`'s Clone bound is somehow required. See: https://github.com/rust-lang/rust/issues/41481
pub struct VrrEntry<Types: ImmutableSchemaAbstractTypes> {
    id: Types::VrrId,
    pk: ApparentPrimaryKey,
    pub(in crate::version_revision_resolver) version_id: VersionId,
    revision: Revision,
}

impl<Types: ImmutableSchemaAbstractTypes> VrrEntry<Types> {
    pub fn into_pk(self) -> ApparentPrimaryKey {
        self.pk
    }
}

impl<Types: ImmutableSchemaAbstractTypes> Clone for VrrEntry<Types> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            pk: self.pk.clone(),
            version_id: self.version_id.clone(),
            revision: self.revision.clone(),
        }
    }
}

impl<Types: ImmutableSchemaAbstractTypes> Entity for VrrEntry<Types> {
    type Id = Types::VrrId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
