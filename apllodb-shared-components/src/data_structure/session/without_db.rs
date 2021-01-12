use serde::{Deserialize, Serialize};

/// Session without open database.
///
/// Only limited SQL commands (`CREATE DATABASE`, for example) are executed via this type of session.
#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct SessionWithoutDb {}
