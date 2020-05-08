use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub(super) struct DummyError;

impl Display for DummyError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unreachable!()
    }
}

impl Error for DummyError {}
