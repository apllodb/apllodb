use crate::{row::immutable_row::ImmutableRow, vtable::VTable};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, BooleanExpression, ColumnDataType, ColumnName,
    ColumnReference, ColumnValue, ComparisonFunction, Constant, Expression, FieldIndex,
    LogicalFunction, Record, SqlConvertible, SqlValue, TableName,
};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Primary key which other components than Storage Engine observes.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ApparentPrimaryKey {
    table_name: TableName,
    pk_column_names: Vec<ColumnName>,

    // real "key" of a record.
    sql_values: Vec<SqlValue>,
}

impl ApparentPrimaryKey {
    /// Get [SqlValue](apllodb_shared_components::SqlValue) from a PK column.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb_shared_components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - `column_name` is not in this PK.
    pub fn get_sql_value(&self, column_name: &ColumnName) -> ApllodbResult<&SqlValue> {
        let target_sql_value = self
            .zipped()
            .iter()
            .find_map(|(cn, sql_value)| {
                if *cn == column_name {
                    Some(*sql_value)
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedColumn,
                    format!("undefined column name in PK: `{:?}`", column_name),
                    None,
                )
            })?;
        Ok(target_sql_value)
    }

    /// Get Rust value from a PK column.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb_shared_components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - `column_name` is not in this PK.
    pub fn get<T: SqlConvertible>(&self, column_name: &ColumnName) -> ApllodbResult<T> {
        let sql_value = self.get_sql_value(column_name)?;
        Ok(sql_value.unpack()?)
    }
}

impl ApparentPrimaryKey {
    pub fn from_table_and_immutable_row(
        vtable: &VTable,
        row: &mut ImmutableRow,
    ) -> ApllodbResult<Self> {
        let apk_cdts = vtable.table_wide_constraints().pk_column_data_types();
        let apk_column_names = apk_cdts
            .iter()
            .map(|cdt| cdt.column_ref().as_column_name().clone())
            .collect::<Vec<ColumnName>>();

        let apk_sql_values = apk_cdts
            .iter()
            .map(|cdt| row.get_sql_value(cdt.column_ref()))
            .collect::<ApllodbResult<Vec<SqlValue>>>()?;

        Ok(Self::new(
            vtable.table_name().clone(),
            apk_column_names,
            apk_sql_values,
        ))
    }

    pub fn from_table_and_record(vtable: &VTable, record: &Record) -> ApllodbResult<Self> {
        let apk_cdts = vtable.table_wide_constraints().pk_column_data_types();
        let apk_column_names = apk_cdts
            .iter()
            .map(|cdt| cdt.column_ref().as_column_name().clone())
            .collect::<Vec<ColumnName>>();
        let apk_sql_values = apk_cdts
            .iter()
            .map(|cdt| {
                record
                    .get_sql_value(&FieldIndex::InColumnReference(cdt.column_ref().clone()))
                    // FIXME less clone
                    .map(|sql_value| sql_value.clone())
                    .map_err(|e| {
                        ApllodbError::new(
                            ApllodbErrorKind::NotNullViolation,
                            format!(
                                "primary key column `{:?}` is not held in this record: `{:#?}` (table `{:#?}`)",
                                cdt.column_ref(),
                                record,
                                vtable.table_name()
                            ),
                            Some(Box::new(e)),
                        )
                    })
            })
            .collect::<ApllodbResult<Vec<SqlValue>>>()?;

        Ok(Self::new(
            vtable.table_name().clone(),
            apk_column_names,
            apk_sql_values,
        ))
    }

    pub fn column_names(&self) -> &[ColumnName] {
        &self.pk_column_names
    }

    pub fn sql_values(&self) -> &[SqlValue] {
        &self.sql_values
    }

    pub fn zipped(&self) -> Vec<(&ColumnName, &SqlValue)> {
        self.pk_column_names.iter().zip(&self.sql_values).collect()
    }

    pub fn into_colvals(self) -> Vec<ColumnValue> {
        let table_name = &self.table_name;

        self.pk_column_names
            .into_iter()
            .zip(self.sql_values)
            .map(|(cn, v)| {
                let colref = ColumnReference::new(table_name.clone(), cn);
                ColumnValue::new(colref, v)
            })
            .collect()
    }

    pub fn column_data_types(&self) -> Vec<ColumnDataType> {
        self.zipped()
            .into_iter()
            .map(|(cname, sql_value)| {
                let column_ref = ColumnReference::new(self.table_name.clone(), cname.clone());
                ColumnDataType::new(column_ref, sql_value.data_type().clone())
            })
            .collect()
    }

    pub fn to_condition_expression(&self) -> ApllodbResult<BooleanExpression> {
        let mut comparisons = self
            .zipped()
            .into_iter()
            .map(|(column_name, sql_value)| {
                let constant_expr = Constant::from(sql_value);
                ComparisonFunction::EqualVariant {
                    left: Box::new(Expression::ColumnReferenceVariant(ColumnReference::new(
                        self.table_name.clone(),
                        column_name.clone(),
                    ))),
                    right: Box::new(Expression::ConstantVariant(constant_expr)),
                }
            })
            .collect::<VecDeque<ComparisonFunction>>();

        let mut boolean_expr = BooleanExpression::ComparisonFunctionVariant(
            comparisons
                .pop_front()
                .expect("ApparentPrimaryKey has at least 1 column value"),
        );
        while let Some(comparison) = comparisons.pop_front() {
            boolean_expr = BooleanExpression::LogicalFunctionVariant(LogicalFunction::AndVariant {
                left: Box::new(boolean_expr),
                right: Box::new(BooleanExpression::ComparisonFunctionVariant(comparison)),
            });
        }

        Ok(boolean_expr)
    }
}
