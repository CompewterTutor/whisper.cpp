use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub enum FrontendError {
    MissingPath {
        kind: &'static str,
    },
    NotFound {
        kind: &'static str,
        path: PathBuf,
    },
    InvalidExtension {
        kind: &'static str,
        path: PathBuf,
        expected: &'static str,
    },
    Io {
        context: &'static str,
        source: std::io::Error,
    },
    Serialization {
        context: &'static str,
        source: serde_json::Error,
    },
}

impl FrontendError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingPath { .. } => "missing_path",
            Self::NotFound { .. } => "path_not_found",
            Self::InvalidExtension { .. } => "invalid_extension",
            Self::Io { .. } => "io_error",
            Self::Serialization { .. } => "serialization_error",
        }
    }
}

impl Display for FrontendError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingPath { kind } => write!(formatter, "{kind} path cannot be empty"),
            Self::NotFound { kind, path } => {
                write!(formatter, "{kind} path does not exist: {}", path.display())
            }
            Self::InvalidExtension {
                kind,
                path,
                expected,
            } => write!(
                formatter,
                "{kind} file has unsupported extension: {} (expected {expected})",
                path.display()
            ),
            Self::Io { context, source } => write!(formatter, "{context}: {source}"),
            Self::Serialization { context, source } => write!(formatter, "{context}: {source}"),
        }
    }
}

impl std::error::Error for FrontendError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

impl From<FrontendError> for ApiError {
    fn from(value: FrontendError) -> Self {
        Self {
            code: value.code().to_owned(),
            message: value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ApiError, FrontendError};

    #[test]
    fn maps_frontend_error_to_api_error() {
        let error = FrontendError::MissingPath { kind: "model" };
        let api_error: ApiError = error.into();

        assert_eq!(api_error.code, "missing_path");
        assert!(api_error.message.contains("model path cannot be empty"));
    }
}
