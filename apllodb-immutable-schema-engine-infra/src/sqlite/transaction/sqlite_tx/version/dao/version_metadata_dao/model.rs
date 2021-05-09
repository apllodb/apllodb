use super::{
    CNAME_COLUMN_DATA_TYPES, CNAME_IS_ACTIVE, CNAME_TABLE_NAME, CNAME_VERSION_CONSTRAINTS,
    CNAME_VERSION_NUMBER,
};
use apllodb_immutable_schema_engine_domain::{
    version::{
        active_version::ActiveVersion, constraints::VersionConstraints,
        version_number::VersionNumber,
    },
    vtable::id::VTableId,
};
use apllodb_shared_components::{ApllodbError, ApllodbResult, Schema, SchemaIndex, SqlState};
use apllodb_storage_engine_interface::{ColumnDataType, Row, RowSchema, TableName};

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

impl VersionMetadataModel {
    pub(super) fn from_row(schema: &RowSchema, row: Row) -> ApllodbResult<Self> {
        let col_not_found = || {
            ApllodbError::new(
                SqlState::SystemError,
                "wrong _version_metadata table's column",
                None,
            )
        };

        let (pos_table_name, _) = schema.index(&SchemaIndex::from(CNAME_TABLE_NAME))?;
        let (pos_version_number, _) = schema.index(&SchemaIndex::from(CNAME_VERSION_NUMBER))?;
        let (pos_column_data_types, _) =
            schema.index(&SchemaIndex::from(CNAME_COLUMN_DATA_TYPES))?;
        let (pos_version_constraints, _) =
            schema.index(&SchemaIndex::from(CNAME_VERSION_CONSTRAINTS))?;
        let (pos_is_active, _) = schema.index(&SchemaIndex::from(CNAME_IS_ACTIVE))?;

        let table_name: String = row.get(pos_table_name)?.ok_or_else(col_not_found)?;
        let version_number: i64 = row.get(pos_version_number)?.ok_or_else(col_not_found)?;
        let column_data_types: String =
            row.get(pos_column_data_types)?.ok_or_else(col_not_found)?;
        let version_constraints: String = row
            .get(pos_version_constraints)?
            .ok_or_else(col_not_found)?;
        let is_active: bool = row.get(pos_is_active)?.ok_or_else(col_not_found)?;

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
    /// # Panics
    ///
    /// if this version is inactive.
    pub(super) fn to_active_version(&self, vtable_id: &VTableId) -> ActiveVersion {
        assert!(
            self.is_active,
            "internal error: version here must be active"
        );

        ActiveVersion::new(
            &vtable_id,
            &self.version_number,
            &self.column_data_types,
            self.version_constraints.clone(),
        )
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
            SqlState::SerializationError,
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
            SqlState::DeserializationError,
            "failed to deserialize a value in _version_metadata table",
            Some(Box::new(e)),
        )
    }
}
