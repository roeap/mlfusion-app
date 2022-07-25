use snafu::Snafu;

/// A specialized `Error` for object store-related errors
#[derive(Debug, Snafu)]
#[allow(missing_docs)]
pub enum MlFusionError {
    #[snafu(display("Generic error: {}", source))]
    Generic {
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    #[snafu(display("Missing data"))]
    MissingData,

    #[snafu(display("Error decoding message: {}", source))]
    Decode { source: prost::DecodeError },
}

impl MlFusionError {
    /// Wraps an external error in an `MlFusionError`.
    pub fn from_external_error(error: Box<dyn ::std::error::Error + Send + Sync>) -> Self {
        Self::Generic { source: error }
    }
}
