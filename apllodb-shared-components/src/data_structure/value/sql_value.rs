use crate::{
    data_structure::{
        data_type::{DataType, data_type_kind::DataTypeKind},
        expression::{
            constant::{CharacterConstant, Constant, NumericConstant},
            Expression,
        },
    },
    error::{kind::ApllodbErrorKind, ApllodbError, ApllodbResult},
    traits::sql_convertible::SqlConvertible,
};
use serde::{Deserialize, Serialize};

pub const SQL_VALUE_NULL: Option<i16> = None;

/// SQL-typed value that is efficiently compressed.
///
/// # Comparing SqlValues
///
/// An SqlValue internally holds [DataTypeKind](crate::DataTypeKind).
/// Each DataTypeKind belongs to a [GeneralDataType](crate::GeneralDataType).
/// [DataTypeKind::SmallInt](crate::DataTypeKind::SmallInt) belongs to [GeneralDataType::Number](crate::GeneralDataType::Number), for example.
///
/// ## *Comparable* and *Ordered* types
///
/// If two SqlValues belong to the same GeneralDataType, these two are **comparable**, meaning equality comparison to them is valid.
/// Also, ordered comparison is valid for values within some GeneralDataType.
/// Such GeneralDataType's and values within one are called **ordered**.
/// **Ordered** is stronger property than **comparable**.
///
/// ## Failures on comparison
///
/// Comparing non-**comparable** values and ordered comparison to non-**ordered** values cause [crate::ApllodbErrorKind::DataException].
///
/// ## Comparison with NULL
///
/// Any [DataTypeKind](crate::DataTypeKind) can be NULLABLE.
/// So any SqlValue can calculate equality- and ordered- comparison with NULL value.
///
/// Equality-comparison and ordered-comparison with NULL is evaluated to NULL.
/// NULL is always evaluated as FALSE in boolean context (, therefore all of `x = NULL`, `x != NULL`, `x < NULL`, `x > NULL` are evaluated to FALSE in boolean context).
#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl std::fmt::Debug for SqlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
SqlValue {{
    data_type: {:?},
    (raw into Expression): {:?}
}}",
            self.data_type,
            Expression::from(self)
        )
    }
}
