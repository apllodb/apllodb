use crate::{
    data_structure::{Constant, DataType, DataTypeKind, Expression, NumericConstant},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
    traits::SqlConvertible,
};
use serde::{Deserialize, Serialize};

/// SQL-typed value that is efficiently compressed.
///
/// A storage engine may (or may not) save `SqlValue`'s serialized instance as-is.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct SqlValue {
    data_type: DataType,
    raw: Vec<u8>,
}

impl SqlValue {
    /// Convert rust_value into `SqlValue`
    ///
    /// # Failures
    ///
    /// - Errors from `T::pack()`.
    pub fn pack<T>(into_type: &DataType, rust_value: &T) -> ApllodbResult<Self>
    where
        T: SqlConvertible,
    {
        let raw = T::pack(into_type, rust_value)?;
        Ok(Self {
            data_type: into_type.clone(),
            raw,
        })
    }

    pub fn unpack<T>(&self) -> ApllodbResult<T>
    where
        T: SqlConvertible,
    {
        T::unpack(&self.data_type, &self.raw)
    }

    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }

    pub fn try_from(expr: &Expression, data_type: &DataType) -> ApllodbResult<Self> {
        match expr {
            Expression::ConstantVariant(v) => match v {
                Constant::NumericConstantVariant(v) => match v {
                    NumericConstant::IntegerConstantVariant(integer_const) => {
                        match data_type.kind() {
                            DataTypeKind::SmallInt => {
                                let v = integer_const.0 as i16;
                                SqlValue::pack(&data_type, &v)
                            }
                            DataTypeKind::Integer => {
                                let v = integer_const.0 as i32;
                                SqlValue::pack(&data_type, &v)
                            }
                            DataTypeKind::BigInt => {
                                let v = integer_const.0;
                                SqlValue::pack(&data_type, &v)
                            }
                        }
                    }
                },
            },
            Expression::ColumnNameVariant(column_name) => Err(ApllodbError::new(
                ApllodbErrorKind::DataException,
                format!(
                    "cannot construct SqlValue from column reference: {}",
                    column_name
                ),
                None,
            )),
            Expression::BooleanExpressionVariant(b) => Err(ApllodbError::new(
                ApllodbErrorKind::FeatureNotSupported,
                format!(
                    "currently constructing SqlValue from BooleanExpression is not supported: {:?}",
                    b
                ),
                None,
            )),
        }
    }
}
