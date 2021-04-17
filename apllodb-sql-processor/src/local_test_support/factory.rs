use crate::{
    attribute::attribute_name::AttributeName,
    correlation::{
        aliased_correlation_name::AliasedCorrelationName, correlation_alias::CorrelationAlias,
        correlation_name::CorrelationName,
    },
    field::{aliased_field_name::AliasedFieldName, field_alias::FieldAlias, field_name::FieldName},
    records::{record::Record, record_schema::RecordSchema, Records},
    sql_processor::{
        ddl::DdlProcessor,
        modification::ModificationProcessor,
        query::{query_executor::QueryExecutor, query_plan::QueryPlan, QueryProcessor},
    },
    SqlProcessorContext,
};
use apllodb_immutable_schema_engine_infra::test_support::session_with_tx;
use apllodb_shared_components::{ApllodbError, ApllodbResult, SchemaIndex};
use apllodb_sql_parser::apllodb_ast;
use apllodb_storage_engine_interface::{ColumnName, MockStorageEngine, TableName};
use std::{collections::HashSet, sync::Arc};

impl QueryProcessor<MockStorageEngine> {
    pub async fn run_directly(
        context: Arc<SqlProcessorContext<MockStorageEngine>>,
        select_command: apllodb_ast::SelectCommand,
    ) -> ApllodbResult<Records> {
        let session = session_with_tx(&context.engine).await?;

        let processor = Self::new(context.clone());
        processor
            .run(session, select_command)
            .await
            .map(|(records, _)| records)
            .map_err(ApllodbError::from)
    }
}

impl ModificationProcessor<MockStorageEngine> {
    pub async fn run_directly(
        context: Arc<SqlProcessorContext<MockStorageEngine>>,
        command: apllodb_ast::Command,
    ) -> ApllodbResult<()> {
        let session = session_with_tx(&context.engine).await?;

        let processor = Self::new(context.clone());
        processor
            .run(session, command)
            .await
            .map(|_| ())
            .map_err(ApllodbError::from)
    }
}

impl DdlProcessor<MockStorageEngine> {
    pub async fn run_directly(
        context: Arc<SqlProcessorContext<MockStorageEngine>>,
        command: apllodb_ast::Command,
    ) -> ApllodbResult<()> {
        let session = session_with_tx(&context.engine).await?;

        let processor = Self::new(context.clone());
        processor
            .run(session, command)
            .await
            .map(|_| ())
            .map_err(ApllodbError::from)
    }
}

impl QueryExecutor<MockStorageEngine> {
    pub async fn run_directly(
        context: Arc<SqlProcessorContext<MockStorageEngine>>,
        plan: QueryPlan,
    ) -> ApllodbResult<Records> {
        let session = session_with_tx(&context.engine).await?;

        let executor = Self::new(context.clone());
        executor
            .run(session, plan)
            .await
            .map(|(records, _)| records)
            .map_err(ApllodbError::from)
    }
}

impl AliasedFieldName {
    pub fn factory(table_name: &str, column_name: &str) -> Self {
        Self::new(FieldName::factory(table_name, column_name), None)
    }

    pub fn with_corr_alias(self, correlation_alias: &str) -> Self {
        let field_name = self.field_name.with_corr_alias(correlation_alias);
        Self::new(field_name, None)
    }

    pub fn with_field_alias(self, field_alias: &str) -> Self {
        let alias = FieldAlias::factory(field_alias);
        Self::new(self.field_name, Some(alias))
    }
}

impl FieldName {
    pub fn factory(table_name: &str, column_name: &str) -> Self {
        Self::new(
            AliasedCorrelationName::factory_tn(table_name),
            AttributeName::factory(column_name),
        )
    }

    pub fn with_corr_alias(self, correlation_alias: &str) -> Self {
        let aliased_correlation_name = self.aliased_correlation_name.with_alias(correlation_alias);
        Self::new(aliased_correlation_name, self.attribute_name)
    }
}

impl FieldAlias {
    pub fn factory(field_alias: &str) -> Self {
        Self::new(field_alias).unwrap()
    }
}

impl AliasedCorrelationName {
    pub fn factory_tn(table_name: &str) -> Self {
        Self::new(CorrelationName::factory(table_name), None)
    }

    pub fn with_alias(self, correlation_alias: &str) -> Self {
        let alias = CorrelationAlias::factory(correlation_alias);
        Self::new(self.correlation_name, Some(alias))
    }
}

impl CorrelationName {
    pub fn factory(table_name: &str) -> Self {
        Self::TableNameVariant(TableName::factory(table_name))
    }
}

impl CorrelationAlias {
    pub fn factory(correlation_alias: &str) -> Self {
        Self::new(correlation_alias).unwrap()
    }
}

impl AttributeName {
    pub fn factory(column_name: &str) -> Self {
        Self::ColumnNameVariant(ColumnName::factory(column_name))
    }
}

impl Records {
    pub fn factory(schema: RecordSchema, records: Vec<Record>) -> Self {
        Self::new(Arc::new(schema), records)
    }
}

impl RecordSchema {
    pub fn factory(aliased_field_names: Vec<AliasedFieldName>) -> Self {
        Self::from(aliased_field_names.into_iter().collect::<HashSet<_>>())
    }

    pub fn joined(&self, right: &Self) -> Self {
        let mut left = self.to_aliased_field_names().to_vec();
        let mut right = right.to_aliased_field_names().to_vec();
        left.append(&mut right);
        Self::factory(left)
    }
}

impl Record {
    pub fn projection(self, indexes: &[SchemaIndex]) -> ApllodbResult<Self> {
        let schema = self.schema.clone();
        let records = Records::new(schema, vec![self]);
        let records = records.projection(indexes)?;
        records.next().ok_or_else(|| unreachable!())
    }
}
