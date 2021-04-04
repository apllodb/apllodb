use std::convert::TryFrom;

use apllodb_immutable_schema_engine_domain::{
    row::immutable_row::ImmutableRow,
    version::{
        active_version::ActiveVersion, constraints::VersionConstraints,
        version_number::VersionNumber,
    },
    vtable::id::VTableId,
};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnDataType, ColumnName, TableName,
};

use super::{
    CNAME_COLUMN_DATA_TYPES, CNAME_IS_ACTIVE, CNAME_TABLE_NAME, CNAME_VERSION_CONSTRAINTS,
    CNAME_VERSION_NUMBER,
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub(super) struct VersionMetadataModel {
    pub(super) table_name: TableName,
    pub(super) version_number: VersionNumber,
    pub(super) column_data_types: Vec<ColumnDataType>,
    pub(super) version_constraints: VersionConstraints,
    pub(super) is_active: bool,
}

impl From<&ActiveVersion> for VersionMetadataModel {
    fn from(v: &ActiveVersion) -> Self {
        Self {
            table_name: v.vtable_id().table_name().clone(),
            version_number: v.number().clone(),
            column_data_types: v.column_data_types().to_vec(),
            version_constraints: v.version_constraints().clone(),
            is_active: true,
        }
    }
}

impl TryFrom<ImmutableRow> for VersionMetadataModel {
    type Error = ApllodbError;

    fn try_from(mut r: ImmutableRow) -> ApllodbResult<Self> {
        let col_not_found = || {
            ApllodbError::new(
                ApllodbErrorKind::SystemError,
                "wrong _version_metadata table's column",
                None,
            )
        };

        let table_name: String = r
            .get(&ColumnName::new(CNAME_TABLE_NAME)?)?
            .ok_or_else(col_not_found)?;
        let version_number: i64 = r
            .get(&ColumnName::new(CNAME_VERSION_NUMBER)?)?
            .ok_or_else(col_not_found)?;
        let column_data_types: String = r
            .get(&ColumnName::new(CNAME_COLUMN_DATA_TYPES)?)?
            .ok_or_else(col_not_found)?;
        let version_constraints: String = r
            .get(&ColumnName::new(CNAME_VERSION_CONSTRAINTS)?)?
            .ok_or_else(col_not_found)?;
        let is_active: bool = r
            .get(&ColumnName::new(CNAME_IS_ACTIVE)?)?
            .ok_or_else(col_not_found)?;

        Ok(Self {
            table_name: TableName::new(table_name)?,
            version_number: VersionNumber::from(version_number),
            column_data_types: Self::deserialize_column_data_types(&column_data_types)?,
            version_constraints: Self::deserialize_version_constraints(&version_constraints)?,
            is_active,
        })
    }
}

impl VersionMetadataModel {
    pub(super) fn into_active_version(&self, vtable_id: &VTableId) -> ApllodbResult<ActiveVersion> {
        if self.is_active {
            ActiveVersion::new(
                &vtable_id,
                &self.version_number,
                &self.column_data_types,
                self.version_constraints.clone(),
            )
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::InvalidVersion,
                format!("not an active version: {:?}", self),
                None,
            ))
        }
    }

    pub(super) fn serialized_table_name(&self) -> String {
        self.table_name.as_str().to_string()
    }
    pub(super) fn serialized_version_number(&self) -> i32 {
        self.version_number.to_u64() as i32
    }
    pub(super) fn serialized_column_data_types(&self) -> ApllodbResult<String> {
        serde_yaml::to_string(&self.column_data_types).map_err(Self::serialization_err)
    }
    pub(super) fn serialized_version_constraints(&self) -> ApllodbResult<String> {
        serde_yaml::to_string(&self.version_constraints).map_err(Self::serialization_err)
    }
    pub(super) fn serialized_is_active(&self) -> bool {
        self.is_active
    }
    fn serialization_err(e: serde_yaml::Error) -> ApllodbError {
        ApllodbError::new(
            ApllodbErrorKind::SerializationError,
            "failed to serialize a value in _version_metadata table",
            Some(Box::new(e)),
        )
    }

    fn deserialize_column_data_types(yml: &str) -> ApllodbResult<Vec<ColumnDataType>> {
        serde_yaml::from_str(&yml).map_err(Self::deserialization_err)
    }
    fn deserialize_version_constraints(yml: &str) -> ApllodbResult<VersionConstraints> {
        serde_yaml::from_str(&yml).map_err(Self::deserialization_err)
    }
    fn deserialization_err(e: serde_yaml::Error) -> ApllodbError {
        ApllodbError::new(
            ApllodbErrorKind::DeserializationError,
            "failed to deserialize a value in _version_metadata table",
            Some(Box::new(e)),
        )
    }
}
