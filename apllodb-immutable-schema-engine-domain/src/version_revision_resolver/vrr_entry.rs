use std::{fmt::Debug, hash::Hash};

use apllodb_shared_components::ApllodbResult;
use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    abstract_types::ImmutableSchemaAbstractTypes,
    entity::Entity,
    row::pk::apparent_pk::ApparentPrimaryKey,
    row::{
        immutable_row::{builder::ImmutableRowBuilder, ImmutableRow},
        pk::full_pk::revision::Revision,
    },
    version::id::VersionId,
};

#[derive(PartialEq, Hash, Debug, new)] // Clone here doesn't work. `Engine`'s Clone bound is somehow required. See: https://github.com/rust-lang/rust/issues/41481
pub struct VRREntry<
    'sess,
    Engine: StorageEngine<'sess>,
    Types: ImmutableSchemaAbstractTypes<'sess, Engine>,
> {
    id: Types::VRRId,
    pk: ApparentPrimaryKey,
    pub(in crate::version_revision_resolver) version_id: VersionId,
    revision: Revision,
}

impl<'sess, Engine: StorageEngine<'sess>, Types: ImmutableSchemaAbstractTypes<'sess, Engine>>
    VRREntry<'sess, Engine, Types>
{
    pub fn into_pk(self) -> ApparentPrimaryKey {
        self.pk
    }

    pub fn into_pk_only_row(self) -> ApllodbResult<ImmutableRow> {
        let mut builder = ImmutableRowBuilder::default();
        for colval in self.pk.into_colvals() {
            builder = builder.add_colval(colval)?;
        }
        builder.build()
    }
}

impl<'sess, Engine: StorageEngine<'sess>, Types: ImmutableSchemaAbstractTypes<'sess, Engine>> Clone
    for VRREntry<'sess, Engine, Types>
{
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            pk: self.pk.clone(),
            version_id: self.version_id.clone(),
            revision: self.revision.clone(),
        }
    }
}

impl<'sess, Engine: StorageEngine<'sess>, Types: ImmutableSchemaAbstractTypes<'sess, Engine>> Entity
    for VRREntry<'sess, Engine, Types>
{
    type Id = Types::VRRId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
