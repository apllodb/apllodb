use crate::{row::immutable_row::ImmutableRow, vtable::VTable};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, BooleanExpression, ColumnDataType, ColumnName,
    ComparisonFunction, Expression, FieldIndex, FullFieldReference, LogicalFunction, NNSqlValue,
    Record, SqlConvertible, SqlValue, TableName,
};
use apllodb_storage_engine_interface::TableColumnReference;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Primary key which other components than Storage Engine observes.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ApparentPrimaryKey {
    table_name: TableName,
    pk_column_names: Vec<ColumnName>,

    // real "key" of a record.
    sql_values: Vec<NNSqlValue>,
}

impl ApparentPrimaryKey {
    /// Get [NNSqlValue](apllodb_shared_components::NNSqlValue) from a PK column.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb_shared_components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - `column_name` is not in this PK.
    pub fn get_sql_value(&self, column_name: &ColumnName) -> ApllodbResult<&NNSqlValue> {
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
            .map(|cdt| cdt.column_name().clone())
            .collect::<Vec<ColumnName>>();

        let apk_sql_values = apk_cdts
            .iter()
            .map(|cdt| {
                let tcr = TableColumnReference::new(
                    vtable.table_name().clone(),
                    cdt.column_name().clone(),
                );

                if let SqlValue::NotNull(sql_value) = row.get_sql_value(&tcr)? {
                    Ok(sql_value)
                } else {
                    panic!("primary key's column must be NOT NULL")
                }
            })
            .collect::<ApllodbResult<Vec<NNSqlValue>>>()?;

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
            .map(|cdt| cdt.column_name().clone())
            .collect::<Vec<ColumnName>>();
        let apk_sql_values = apk_cdts
            .iter()
            .map(|cdt| {
                let tcr = TableColumnReference::new(
                    vtable.table_name().clone(),
                    cdt.column_name().clone(),
                );
                let ffr = FullFieldReference::from(tcr);

                record
                    .get_sql_value(&FieldIndex::InFullFieldReference(ffr))
                    // FIXME less clone
                    .map(|sql_value| {
                        if let SqlValue::NotNull(sql_value) =    sql_value {
                            sql_value.clone()
                        }else {
                            panic!("primary key's column must be NOT NULL")
                        }
                    }
                    )
                    .map_err(|e| {
                        ApllodbError::new(
                            ApllodbErrorKind::NotNullViolation,
                            format!(
                                "primary key column `{:?}` is not held in this record: `{:#?}` (table `{:#?}`)",
                                cdt.column_name(),
                                record,
                                vtable.table_name()
                            ),
                            Some(Box::new(e)),
                        )
                    })
            })
            .collect::<ApllodbResult<Vec<NNSqlValue>>>()?;

        Ok(Self::new(
            vtable.table_name().clone(),
            apk_column_names,
            apk_sql_values,
        ))
    }

    pub fn table_name(&self) -> &TableName {
        &self.table_name
    }

    pub fn column_names(&self) -> &[ColumnName] {
        &self.pk_column_names
    }

    pub fn sql_values(&self) -> &[NNSqlValue] {
        &self.sql_values
    }

    pub fn zipped(&self) -> Vec<(&ColumnName, &NNSqlValue)> {
        self.pk_column_names.iter().zip(&self.sql_values).collect()
    }

    pub fn into_zipped(self) -> Vec<(ColumnName, NNSqlValue)> {
        self.pk_column_names
            .into_iter()
            .zip(self.sql_values)
            .collect()
    }

    pub fn column_data_types(&self) -> Vec<ColumnDataType> {
        self.zipped()
            .into_iter()
            .map(|(cname, sql_value)| {
                ColumnDataType::new(cname.clone(), sql_value.sql_type(), false)
            })
            .collect()
    }

    pub fn to_condition_expression(&self) -> ApllodbResult<BooleanExpression> {
        let mut comparisons = self
            .zipped()
            .into_iter()
            .map(|(column_name, sql_value)| {
                let tcr = TableColumnReference::new(self.table_name.clone(), column_name.clone());
                ComparisonFunction::EqualVariant {
                    left: Box::new(Expression::FullFieldReferenceVariant(
                        FullFieldReference::from(tcr),
                    )),
                    right: Box::new(Expression::ConstantVariant(SqlValue::NotNull(
                        sql_value.clone(),
                    ))),
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
