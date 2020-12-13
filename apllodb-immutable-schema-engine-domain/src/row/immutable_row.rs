pub mod builder;

use apllodb_shared_components::{
    data_structure::{ColumnReference, ColumnValue, SqlValue},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_engine_interface::{Row, StorageEngine};
use std::collections::{hash_map::Entry, HashMap};

use crate::{
    abstract_types::ImmutableSchemaAbstractTypes, version_revision_resolver::vrr_entry::VRREntry,
};

/// Immutable row which is never updated or deleted by any transaction.
/// Only used for SELECT statement (or internally for UPDATE == SELECT + INSERT).
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ImmutableRow {
    col_vals: HashMap<ColumnReference, SqlValue>,
    // TODO have TransactionId to enable time-machine (TODO naming...) feature.
}

impl Row for ImmutableRow {
    fn get_sql_value(&mut self, colref: &ColumnReference) -> ApllodbResult<SqlValue> {
        self.col_vals.remove(&colref).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedColumn,
                format!("undefined column: `{}`", colref),
                None,
            )
        })
    }

    fn append(&mut self, colvals: Vec<ColumnValue>) -> ApllodbResult<()> {
        colvals
            .into_iter()
            .map(
                |colval| match self.col_vals.entry(colval.as_column_ref().clone()) {
                    Entry::Occupied(_) => Err(ApllodbError::new(
                        ApllodbErrorKind::DuplicateColumn,
                        format!("column `{}` is already in this row", colval.as_column_ref()),
                        None,
                    )),
                    Entry::Vacant(e) => {
                        e.insert(colval.into_sql_value());
                        Ok(())
                    }
                },
            )
            .collect::<ApllodbResult<Vec<()>>>()?;

        Ok(())
    }
}

impl<
        'vrr,
        'db: 'vrr,
        Engine: StorageEngine<'vrr, 'db>,
        Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
    > From<VRREntry<'vrr, 'db, Engine, Types>> for ImmutableRow
{
    fn from(_: VRREntry<'vrr, 'db, Engine, Types>) -> Self {
        todo!()
    }
}
