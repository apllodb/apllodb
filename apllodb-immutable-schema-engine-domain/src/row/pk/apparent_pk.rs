use crate::vtable::VTable;
use apllodb_shared_components::{
    ApllodbError, ApllodbResult, BooleanExpression, ComparisonFunction, Expression,
    LogicalFunction, NnSqlValue, RPos, Schema, SchemaIndex, SqlConvertible, SqlValue,
};
use apllodb_storage_engine_interface::{ColumnDataType, ColumnName, Row, RowSchema, TableName};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, ops::Index};

/// Primary key which other components than Storage Engine observes.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ApparentPrimaryKey {
    table_name: TableName,
    pk_column_names: Vec<ColumnName>,

    // real "key" of a record.
    sql_values: Vec<NnSqlValue>,
}

impl ApparentPrimaryKey {
    /// Get [NnSqlValue](apllodb_shared_components::NnSqlValue) from a PK column.
    ///
    /// # Failures
    ///
    /// - [NameErrorNotFound](apllodb_shared_components::SqlState::NameErrorNotFound) when:
    ///   - `column_name` is not in this PK.
    pub fn get_sql_value(&self, column_name: &ColumnName) -> ApllodbResult<&NnSqlValue> {
        let target_sql_value = self
            .zipped()
            .iter()
            .find_map(|(cn, sql_value)| (*cn == column_name).then(|| *sql_value))
            .ok_or_else(|| {
                ApllodbError::name_error_not_found(format!(
                    "undefined column name in PK: `{:?}`",
                    column_name
                ))
            })?;
        Ok(target_sql_value)
    }

    /// Get Rust value from a PK column.
    ///
    /// # Failures
    ///
    /// - [NameErrorNotFound](apllodb_shared_components::SqlState::NameErrorNotFound) when:
    ///   - `column_name` is not in this PK.
    pub fn get<T: SqlConvertible>(&self, column_name: &ColumnName) -> ApllodbResult<T> {
        let sql_value = self.get_sql_value(column_name)?;
        sql_value.unpack()
    }
}

impl ApparentPrimaryKey {
    pub fn from_table_and_row(
        vtable: &VTable,
        schema: &RowSchema,
        row: &mut Row,
    ) -> ApllodbResult<Self> {
        let apk_cdts = vtable.table_wide_constraints().pk_column_data_types();
        let apk_column_names = apk_cdts
            .iter()
            .map(|cdt| cdt.column_name().clone())
            .collect::<Vec<ColumnName>>();

        let apk_sql_values = apk_cdts
            .iter()
            .map(|cdt| {
                let (pos, _) = schema.index(&SchemaIndex::from(cdt.column_name()))?;
                if let SqlValue::NotNull(sql_value) = row.get_sql_value(pos)? {
                    Ok(sql_value.clone())
                } else {
                    panic!("primary key's column must be NOT NULL")
                }
            })
            .collect::<ApllodbResult<Vec<NnSqlValue>>>()?;

        Ok(Self::new(
            vtable.table_name().clone(),
            apk_column_names,
            apk_sql_values,
        ))
    }

    pub fn from_table_pk_def(
        vtable: &VTable,
        column_names: &[ColumnName],
        row: &Row,
    ) -> ApllodbResult<Self> {
        let apk_cdts = vtable.table_wide_constraints().pk_column_data_types();
        let apk_column_names = apk_cdts
            .iter()
            .map(|cdt| cdt.column_name().clone())
            .collect::<Vec<ColumnName>>();
        let apk_sql_values = apk_cdts
            .iter()
            .map(|cdt| {
                let raw_pos = column_names
                    .iter()
                    .position(|cn| cn == cdt.column_name())
                    .unwrap_or_else(|| {
                        panic!(
                            "primary key's column `{}` is not included in PK's columns=`{:#?}`",
                            cdt.column_name().as_str(),
                            apk_cdts
                        )
                    });
                let sql_value = row.index(RPos::new(raw_pos)).clone();
                if let SqlValue::NotNull(nn_sql_value) = sql_value {
                    nn_sql_value
                } else {
                    panic!("primary key's column must be NOT NULL")
                }
            })
            .collect::<Vec<NnSqlValue>>();

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

    pub fn sql_values(&self) -> &[NnSqlValue] {
        &self.sql_values
    }

    pub fn zipped(&self) -> Vec<(&ColumnName, &NnSqlValue)> {
        self.pk_column_names.iter().zip(&self.sql_values).collect()
    }

    pub fn into_zipped(self) -> Vec<(ColumnName, NnSqlValue)> {
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
                let index = SchemaIndex::from(
                    format!("{}.{}", self.table_name.as_str(), column_name.as_str()).as_str(),
                );
                ComparisonFunction::EqualVariant {
                    left: Box::new(Expression::SchemaIndexVariant(index)),
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
