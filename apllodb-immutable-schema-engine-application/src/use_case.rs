pub mod transaction;

use apllodb_shared_components::error::ApllodbResult;
use log::*;
use std::{any::type_name, fmt::Debug};

pub trait UseCaseInput: Debug {}
pub trait UseCaseOutput: Debug {}

pub trait UseCase {
    type In: UseCaseInput;
    type Out: UseCaseOutput;

    #[doc(hidden)]
    fn run_core(input: Self::In) -> ApllodbResult<Self::Out>;

    fn run(input: Self::In) -> ApllodbResult<Self::Out> {
        debug!("{}::run() input: {:?}", type_name::<Self>(), &input);

        Self::run_core(input)
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
