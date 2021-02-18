use crate::{row::immutable_row::ImmutableRow, vtable::VTable};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, BooleanExpression, ColumnDataType, ColumnName,
    ComparisonFunction, CorrelationName, Expression, FieldReference, FullFieldReference,
    LogicalFunction, NNSqlValue, SqlConvertible, SqlValue, SqlValues, TableName,
};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, ops::Index};

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
                if let SqlValue::NotNull(sql_value) = row.get_sql_value(cdt.column_name())? {
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

    pub fn from_table_pk_def(
        vtable: &VTable,
        column_names: &[ColumnName],
        sql_values: &SqlValues,
    ) -> ApllodbResult<Self> {
        let apk_cdts = vtable.table_wide_constraints().pk_column_data_types();
        let apk_column_names = apk_cdts
            .iter()
            .map(|cdt| cdt.column_name().clone())
            .collect::<Vec<ColumnName>>();
        let apk_sql_values = apk_cdts
            .iter()
            .map(|cdt| {
                let idx = column_names
                    .iter()
                    .position(|cn| cn == cdt.column_name())
                    .unwrap_or_else(|| {
                        panic!(format!(
                            "primary key's column `{}` is not inclueded in PK's columns=`{:#?}`",
                            cdt.column_name().as_str(),
                            apk_cdts
                        ))
                    });
                let sql_value = sql_values.index(idx).clone();
                if let SqlValue::NotNull(nn_sql_value) = sql_value {
                    nn_sql_value
                } else {
                    panic!("primary key's column must be NOT NULL")
                }
            })
            .collect::<Vec<NNSqlValue>>();

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
                let ffr = FullFieldReference::new(
                    Some(CorrelationName::TableNameVariant(
                        self.table_name.clone(),
                    )),
                    FieldReference::ColumnNameVariant(column_name.clone()),
                );
                ComparisonFunction::EqualVariant {
                    left: Box::new(Expression::UnresolvedFieldReferenceVariant(ffr)),
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
