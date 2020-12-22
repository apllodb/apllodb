use crate::{
    data_structure::{
        CharacterConstant, Constant, DataType, DataTypeKind, Expression, IntegerConstant,
        NumericConstant, TextConstant,
    },
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
    traits::SqlConvertible,
};
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

    pub fn null() -> Self {
        Self::pack(
            &DataType::new(DataTypeKind::SmallInt, true),
            &SQL_VALUE_NULL,
        )
        .expect("creating NULL should always succeed")
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
                Constant::NumericConstantVariant(v) => match v {
                    NumericConstant::IntegerConstantVariant(IntegerConstant(i)) => {
                        match data_type.kind() {
                            DataTypeKind::SmallInt => {
                                let v = *i as i16;
                                SqlValue::pack(&data_type, &v)
                            }
                            DataTypeKind::Integer => {
                                let v = *i as i32;
                                SqlValue::pack(&data_type, &v)
                            }
                            DataTypeKind::BigInt => {
                                let v = *i;
                                SqlValue::pack(&data_type, &v)
                            }
                            data_type_kind  => {
                                Err(ApllodbError::new(
                                    ApllodbErrorKind::DatatypeMismatch,
                                    format!("expression `{:?}` is integer constant but requested to interpret as {:?}", v, data_type_kind),
                                    None
                                ))
                            }
                        }
                    }
                },
                Constant::CharacterConstantVariant(c) => match c {
                    CharacterConstant::TextConstantVariant(TextConstant(s)) => {
                        match data_type.kind() {
                            DataTypeKind::Text => {
                                SqlValue::pack(&data_type, s)
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
