pub mod transaction;

use apllodb_immutable_schema_engine_domain::abstract_types::ImmutableSchemaAbstractTypes;
use apllodb_shared_components::error::ApllodbResult;
use apllodb_storage_engine_interface::StorageEngine;
use log::*;
use std::{any::type_name, fmt::Debug};

pub trait UseCaseInput: Debug {
    fn validate(&self) -> ApllodbResult<()>;
}
pub trait UseCaseOutput: Debug {}

pub trait UseCase<Engine: StorageEngine, Types: ImmutableSchemaAbstractTypes<Engine>> {
    type In: UseCaseInput;
    type Out: UseCaseOutput;

    #[doc(hidden)]
    fn run_core(tx: &Types::ImmutableSchemaTx, input: Self::In) -> ApllodbResult<Self::Out>;

    fn run(tx: &Types::ImmutableSchemaTx, input: Self::In) -> ApllodbResult<Self::Out> {
        debug!(
            "{}::run() tx: {:?} ; input: {:?}",
            type_name::<Self>(),
            tx,
            &input
        );

        input.validate()?;

        Self::run_core(tx, input)
            .map_err(|e| {
                debug!("{}::run() raised error: {:?}", type_name::<Self>(), e);
                e
            })
            .map(|out| {
                debug!(
                    "{}::run() succeeds with output: {:?}",
                    type_name::<Self>(),
                    out
                );
                out
            })
    }
}
