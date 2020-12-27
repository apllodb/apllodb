pub mod transaction;

use apllodb_immutable_schema_engine_domain::abstract_types::ImmutableSchemaAbstractTypes;
use apllodb_shared_components::ApllodbResult;
use apllodb_storage_engine_interface::StorageEngine;
use log::*;
use std::{any::type_name, fmt::Debug};

pub trait UseCaseInput: Debug {
    fn validate(&self) -> ApllodbResult<()>;
}
pub trait UseCaseOutput: Debug {}

/// Usecase using [Transaction](apllodb-storage-engine-interface::Transaction).
pub trait TxUseCase<
    'usecase,
    'db: 'usecase,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
>
{
    type In: UseCaseInput;
    type Out: UseCaseOutput;

    #[doc(hidden)]
    fn run_core(
        vtable_repo: &Types::VTableRepo,
        version_repo: &Types::VersionRepo,
        input: Self::In,
    ) -> ApllodbResult<Self::Out>;

    fn run(
        vtable_repo: &Types::VTableRepo,
        version_repo: &Types::VersionRepo,
        input: Self::In,
    ) -> ApllodbResult<Self::Out> {
        debug!("{}::run() input: {:#?}", type_name::<Self>(), &input);

        input.validate()?;

        Self::run_core(vtable_repo, version_repo, input)
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
