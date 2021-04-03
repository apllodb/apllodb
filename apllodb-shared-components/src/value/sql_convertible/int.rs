use crate::{ApllodbResult, NnSqlValue};

use super::SqlConvertible;

impl SqlConvertible for i16 {
    fn into_sql_value(self) -> NnSqlValue {
        NnSqlValue::SmallInt(self)
    }

    fn try_from_i16(v: &i16) -> ApllodbResult<Self> {
        Ok(*v)
    }
}

impl SqlConvertible for i32 {
    fn into_sql_value(self) -> NnSqlValue {
        NnSqlValue::Integer(self)
    }

    fn try_from_i16(v: &i16) -> ApllodbResult<Self> {
        Ok(*v as i32)
    }

    fn try_from_i32(v: &i32) -> ApllodbResult<Self> {
        Ok(*v)
    }
}

impl SqlConvertible for i64 {
    fn into_sql_value(self) -> NnSqlValue {
        NnSqlValue::BigInt(self)
    }

    fn try_from_i16(v: &i16) -> ApllodbResult<Self> {
        Ok(*v as i64)
    }

    fn try_from_i32(v: &i32) -> ApllodbResult<Self> {
        Ok(*v as i64)
    }

    fn try_from_i64(v: &i64) -> ApllodbResult<Self> {
        Ok(*v)
    }
}

#[cfg(test)]
mod tests_i32 {
    use crate::{error::ApllodbResult, NnSqlValue};

    #[test]
    fn test_pack_unpack() -> ApllodbResult<()> {
        let rust_values = vec![0, 1, -1, i32::MAX, i32::MIN];

        for v in rust_values {
            let sql_value = NnSqlValue::Integer(v);
            let unpacked: i32 = sql_value.unpack()?;
            assert_eq!(unpacked, v);
        }
        Ok(())
    }
}
