use std::{convert::TryFrom, fmt::Debug, hash::Hash};

use apllodb_shared_components::{
    ApllodbResult, BooleanExpression, ComparisonFunction, Expression, LogicalFunction, NnSqlValue,
    SchemaIndex, SqlValue,
};
use apllodb_storage_engine_interface::ColumnName;

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

    pub fn to_condition_expression(
        &self,
        revision_column_name: &ColumnName,
    ) -> ApllodbResult<BooleanExpression> {
        let apk_condition = self.pk.to_condition_expression()?;

        let revision_condition = self.to_revision_condition(revision_column_name);

        let apk_and_revison_condition =
            BooleanExpression::LogicalFunctionVariant(LogicalFunction::AndVariant {
                left: Box::new(apk_condition),
                right: Box::new(revision_condition),
            });

        Ok(apk_and_revison_condition)
    }

    fn to_revision_condition(&self, revision_column_name: &ColumnName) -> BooleanExpression {
        let index = SchemaIndex::from(
            format!(
                "{}.{}",
                self.pk.table_name().as_str(),
                revision_column_name.as_str()
            )
            .as_str(),
        );

        let rev: i64 = TryFrom::try_from(self.revision.to_u64())
            .unwrap_or_else(|_| panic!("too large revision number: {:#?}", self));
        let sql_value = SqlValue::NotNull(NnSqlValue::BigInt(rev));

        BooleanExpression::ComparisonFunctionVariant(ComparisonFunction::EqualVariant {
            left: Box::new(Expression::SchemaIndexVariant(index)),
            right: Box::new(Expression::ConstantVariant(sql_value)),
        })
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
