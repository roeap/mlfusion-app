use serde::{Deserialize, Serialize};
use snafu::Snafu;

/// A specialized `Error` for object store-related errors
#[derive(Debug, Snafu)]
#[allow(missing_docs)]
pub enum MlFusionError {
    #[snafu(display("Generic error: {}", source))]
    Generic {
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}

impl MlFusionError {
    /// Wraps an external error in an `MlFusionError`.
    pub fn from_external_error(error: Box<dyn ::std::error::Error + Send + Sync>) -> Self {
        Self::Generic { source: error }
    }
}
