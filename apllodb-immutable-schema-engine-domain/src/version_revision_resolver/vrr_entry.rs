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

#[derive(Eq, PartialEq, Hash, Debug, new)] // Clone here doesn't work. `Engine`'s Clone bound is somehow required. See: https://github.com/rust-lang/rust/issues/41481
pub struct VRREntry<
    'vrr,
    'db: 'vrr,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
> {
    id: Types::VRRId,
    pk: ApparentPrimaryKey,
    pub(in crate::version_revision_resolver) version_id: VersionId,
    revision: Revision,
}

impl<
        'vrr,
        'db: 'vrr,
        Engine: StorageEngine,
        Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
    > VRREntry<'vrr, 'db, Engine, Types>
{
    pub fn into_pk(self) -> ApparentPrimaryKey {
        self.pk
    }

    pub fn into_pk_only_row(self) -> ApllodbResult<ImmutableRow> {
        let mut builder = ImmutableRowBuilder::default();
        for colval in self.pk.into_colvals() {
            let colref = colval.as_column_ref().clone();
            builder = builder.add_col_val(&colref, colval.into_sql_value())?;
        }
        builder.build()
    }
}

impl<
        'vrr,
        'db: 'vrr,
        Engine: StorageEngine,
        Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
    > Clone for VRREntry<'vrr, 'db, Engine, Types>
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

impl<
        'vrr,
        'db: 'vrr,
        Engine: StorageEngine,
        Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
    > Entity for VRREntry<'vrr, 'db, Engine, Types>
{
    type Id = Types::VRRId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
