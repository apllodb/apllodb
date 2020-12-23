use crate::{data_structure::{column::{data_type::DataType, data_type_kind::DataTypeKind}, expression::{Expression, constant::{CharacterConstant, Constant, NumericConstant}}}, error::{kind::ApllodbErrorKind, ApllodbError, ApllodbResult}, traits::sql_convertible::SqlConvertible};
use serde::{Deserialize, Serialize};

pub const SQL_VALUE_NULL: Option<i16> = None;

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

    /// Makes NULL SqlValue
    pub fn null() -> Self {
        Self::pack(
            &DataType::new(DataTypeKind::SmallInt, true),
            &SQL_VALUE_NULL,
        )
        .expect("creating NULL should always succeed")
    }

    /// Retrieve Rust value
    pub fn unpack<T>(&self) -> ApllodbResult<T>
    where
        T: SqlConvertible,
    {
        T::unpack(&self.data_type, &self.raw)
    }

    /// DataType of this value
    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }

    /// Construct from Expression. DataType must be passed explicitly.
    pub fn try_from(expr: &Expression, data_type: &DataType) -> ApllodbResult<Self> {
        match expr {
            Expression::ConstantVariant(v) => match v {
                Constant::Null => {
                    if data_type.nullable() {
                        SqlValue::pack(&data_type, &None::<i16>)
                    } else {
                        Err(ApllodbError::new(
                        ApllodbErrorKind::NullValueNotAllowed,
                        format!("NULL expression is passed but requested to interpret as non-nullable type: {:?}",  data_type),
                        None
                    ))
                    }
                }
                Constant::NumericConstantVariant(nv) => match nv {
                    NumericConstant::IntegerConstantVariant(iv) => 
                        match data_type.kind() {
                            DataTypeKind::SmallInt => {
                                let i = iv.as_i64() as i16;
                                SqlValue::pack(&data_type, &i)
                            }
                            DataTypeKind::Integer => {
                                let i = iv.as_i64() as i32;
                                SqlValue::pack(&data_type, &i)
                            }
                            DataTypeKind::BigInt => {
                                let i = iv.as_i64();
                                SqlValue::pack(&data_type, &i)
                            }
                            data_type_kind  => {
                                Err(ApllodbError::new(
                                    ApllodbErrorKind::DatatypeMismatch,
                                    format!("expression `{:?}` is integer constant but requested to interpret as {:?}", v, data_type_kind),
                                    None
                                ))
                            }
                        }

                },
                Constant::CharacterConstantVariant(c) => match c {
                    CharacterConstant::TextConstantVariant(tx) => {
                        match data_type.kind() {
                            DataTypeKind::Text => {
                                SqlValue::pack(&data_type, &tx.as_str().to_string())
                            }
                            data_type_kind  => {
                                Err(ApllodbError::new(
                                    ApllodbErrorKind::DatatypeMismatch,
                                    format!("expression `{:?}` is character constant but requested to interpret as {:?}", v, data_type_kind),
                                    None
                                ))
                            }
                        }
                    }
                },
            },
            Expression::ColumnNameVariant(column_name) => Err(ApllodbError::new(
                ApllodbErrorKind::DataException,
                format!(
                    "cannot construct SqlValue from column reference: {:?}",
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
