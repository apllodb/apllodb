use std::convert::TryFrom;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    data_structure::reference::correlation_reference::CorrelationReference, ApllodbError,
    ApllodbResult, FieldIndex,
};

use super::field_reference::FieldReference;

/// Full field reference == "correlation.field".
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct FullFieldReference {
    correlation_reference: CorrelationReference,
    field_reference: FieldReference,
}

impl Display for FullFieldReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.correlation_reference, self.field_reference)
    }
}

impl TryFrom<FieldIndex> for FullFieldReference {
    type Error = ApllodbError;

    /// # Panics
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - this field index does not represent a valid field.
    fn try_from(field: FieldIndex) -> ApllodbResult<Self> {
        match field {
            FieldIndex::InFullFieldReference(ffr) => Ok(ffr),
        }
    }
}
