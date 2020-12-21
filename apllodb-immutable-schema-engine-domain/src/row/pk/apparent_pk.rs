use crate::{row::immutable_row::ImmutableRow, vtable::VTable};
use apllodb_shared_components::{
    data_structure::{
        BooleanExpression, ColumnDataType, ColumnName, ColumnReference, ColumnValue,
        ComparisonFunction, Constant, Expression, LogicalFunction, SqlValue, TableName,
    },
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_engine_interface::{PrimaryKey, Row};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Primary key which other components than Storage Engine observes.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ApparentPrimaryKey {
    table_name: TableName,
    pk_column_names: Vec<ColumnName>,

    // real "key" of a record.
    sql_values: Vec<SqlValue>,
}

impl PrimaryKey for ApparentPrimaryKey {
    fn get_core(&self, column_name: &ColumnName) -> ApllodbResult<&SqlValue> {
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
                    format!("undefined column name in PK: `{}`", column_name),
                    None,
                )
            })?;
        Ok(target_sql_value)
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

    pub fn from_table_and_column_values(
        vtable: &VTable,
        column_values: &HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<Self> {
        let apk_cdts = vtable.table_wide_constraints().pk_column_data_types();
        let apk_column_names = apk_cdts
            .iter()
            .map(|cdt| cdt.column_ref().as_column_name().clone())
            .collect::<Vec<ColumnName>>();
        let apk_sql_values = apk_cdts
            .iter()
            .map(|cdt| {
                let expr = column_values
                    .get(cdt.column_ref().as_column_name())
                    .ok_or_else(|| {
                        ApllodbError::new(
                            ApllodbErrorKind::NotNullViolation,
                            format!(
                                "primary key column `{}` must be specified (table `{}`)",
                                cdt.column_ref(),
                                vtable.table_name()
                            ),
                            None,
                        )
                    })?;
                SqlValue::try_from(expr, cdt.data_type())
            })
            .collect::<ApllodbResult<Vec<SqlValue>>>()?;

        Ok(Self::new(
            vtable.table_name().clone(),
            apk_column_names,
            apk_sql_values,
        ))
    }

    /// Returns old value.
    pub fn update_colval(&mut self, _col: &ColumnName, _val: SqlValue) -> ApllodbResult<SqlValue> {
        todo!()
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
                    left: Box::new(Expression::ColumnNameVariant(column_name.clone())),
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
