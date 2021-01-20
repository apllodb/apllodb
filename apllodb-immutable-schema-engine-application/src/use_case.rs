pub mod transaction;

use apllodb_immutable_schema_engine_domain::abstract_types::ImmutableSchemaAbstractTypes;
use apllodb_shared_components::ApllodbResult;
use apllodb_storage_engine_interface::StorageEngine;
use async_trait::async_trait;
use log::*;
use std::{any::type_name, fmt::Debug};

pub trait UseCaseInput: Debug {
    fn validate(&self) -> ApllodbResult<()>;
}
pub trait UseCaseOutput: Debug {}

/// Usecase using [Transaction](apllodb-storage-engine-interface::Transaction).
#[async_trait(?Send)]
pub trait TxUseCase<Engine: StorageEngine, Types: ImmutableSchemaAbstractTypes<Engine>> {
    type In: UseCaseInput;
    type Out: UseCaseOutput;

    #[doc(hidden)]
    async fn run_core(
        vtable_repo: &Types::VTableRepo,
        version_repo: &Types::VersionRepo,
        input: Self::In,
    ) -> ApllodbResult<Self::Out>;

    async fn run(
        vtable_repo: &Types::VTableRepo,
        version_repo: &Types::VersionRepo,
        input: Self::In,
    ) -> ApllodbResult<Self::Out>
    where
        Engine: 'async_trait,
        Types: 'async_trait,
    {
        debug!("{}::run() input: {:#?}", type_name::<Self>(), &input);

        input.validate()?;

        Self::run_core(vtable_repo, version_repo, input)
            .await
            .map_err(|e| {
                debug!("{}::run() raised error: {:#?}", type_name::<Self>(), e);
                e
            })
            .map(|out| {
                debug!(
                    "{}::run() succeeds with output: {:#?}",
                    type_name::<Self>(),
                    out
                );
                out
            })
    }
}
