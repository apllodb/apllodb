pub(crate) mod character_constant;
pub(crate) mod numeric_constant;

use serde::{Deserialize, Serialize};

use crate::{
    data_structure::{data_type::data_type_kind::DataTypeKind, value::sql_value::SqlValue},
    error::kind::ApllodbErrorKind,
    traits::sql_convertible::SqlConvertible,
};

use self::{character_constant::CharacterConstant, numeric_constant::NumericConstant};

/// Constant.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Constant {
    /// NULL
    Null,

    /// Numeric constant.
    NumericConstantVariant(NumericConstant),

    /// Character(s) constant.
    CharacterConstantVariant(CharacterConstant),
}

impl From<&SqlValue> for Constant {
    fn from(sql_value: &SqlValue) -> Self {
        fn core<T: SqlConvertible, F: FnOnce(T) -> Constant>(
            sql_value: &SqlValue,
            rust_val_to_constant: F,
        ) -> Constant {
            let msg = "DatatypeMismatch never happen if pattern-match here is correct";

            match sql_value.unpack::<Option<T>>() {
                Ok(opt) => match opt {
                    None => Constant::Null,
                    Some(v) => rust_val_to_constant(v),
                },
                Err(e) => match e.kind() {
                    ApllodbErrorKind::DatatypeMismatch => {
                        let v = sql_value.unpack::<T>().expect(msg);
                        rust_val_to_constant(v)
                    }
                    _ => panic!("unexpected error `{}` with sql_value=`{:?}`", e, sql_value),
                },
            }
        }

        match sql_value.data_type().kind() {
            DataTypeKind::SmallInt => core::<i16, _>(sql_value, Self::from),
            DataTypeKind::Integer => core::<i32, _>(sql_value, Self::from),
            DataTypeKind::BigInt => core::<i64, _>(sql_value, Self::from),
            DataTypeKind::Text => core::<String, _>(sql_value, Self::from),
        }
    }
}
