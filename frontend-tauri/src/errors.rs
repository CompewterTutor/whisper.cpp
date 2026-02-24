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
    ExecutionTimeout {
        timeout_ms: u64,
    },
    ExecutionCancelled,
}

impl FrontendError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingPath { .. } => "missing_path",
            Self::NotFound { .. } => "path_not_found",
            Self::InvalidExtension { .. } => "invalid_extension",
            Self::Io { .. } => "io_error",
            Self::Serialization { .. } => "serialization_error",
            Self::ExecutionTimeout { .. } => "execution_timeout",
            Self::ExecutionCancelled => "execution_cancelled",
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
            Self::ExecutionTimeout { timeout_ms } => {
                write!(formatter, "execution exceeded timeout of {timeout_ms} ms")
            }
            Self::ExecutionCancelled => write!(formatter, "execution cancelled before start"),
        }
    }
}

impl std::error::Error for FrontendError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

/// Returns an actionable recovery hint based on the error code.
fn recovery_hint_for_code(code: &str) -> Option<&'static str> {
    match code {
        "missing_path" => Some("Provide a valid file path in the input field."),
        "path_not_found" => Some(
            "Verify the file exists and the path is correct. Use the Browse button to select a file.",
        ),
        "invalid_extension" => Some(
            "Select a file with the correct extension. Models require .bin files; audio files must be wav, mp3, flac, ogg, or m4a.",
        ),
        "io_error" => {
            Some("Check file permissions and ensure the file is not locked by another application.")
        }
        "execution_timeout" => Some(
            "Try a smaller model or shorter audio file. You can also increase the timeout in settings.",
        ),
        "execution_cancelled" => Some("The operation was cancelled. Click Run to try again."),
        "selection_cancelled" => Some("File selection was cancelled. Click Browse to try again."),
        "shortcut_register_failed" => Some(
            "The shortcut may conflict with another application. Try a different key combination.",
        ),
        "shortcut_unregister_failed" => {
            Some("Restart the application if shortcuts behave unexpectedly.")
        }
        _ => None,
    }
}

impl From<FrontendError> for ApiError {
    fn from(value: FrontendError) -> Self {
        let code = value.code().to_owned();
        let hint = recovery_hint_for_code(&code).map(|s| s.to_owned());
        Self {
            code,
            message: value.to_string(),
            hint,
        }
    }
}

impl ApiError {
    /// Creates an ApiError with an explicit code and message, deriving hint automatically.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        let code = code.into();
        let hint = recovery_hint_for_code(&code).map(|s| s.to_owned());
        Self {
            code,
            message: message.into(),
            hint,
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
        assert!(api_error.hint.is_some());
    }

    #[test]
    fn api_error_includes_hint_for_known_error_codes() {
        let error = FrontendError::NotFound {
            kind: "model",
            path: std::path::PathBuf::from("/nonexistent.bin"),
        };
        let api_error: ApiError = error.into();

        assert!(api_error.hint.is_some());
        assert!(api_error.hint.as_ref().unwrap().contains("Browse button"));
    }

    #[test]
    fn api_error_new_derives_hint_automatically() {
        let api_error = ApiError::new("path_not_found", "File not found");

        assert_eq!(api_error.code, "path_not_found");
        assert_eq!(api_error.message, "File not found");
        assert!(api_error.hint.is_some());
    }

    #[test]
    fn api_error_new_returns_none_hint_for_unknown_codes() {
        let api_error = ApiError::new("unknown_error_code", "Something went wrong");

        assert_eq!(api_error.code, "unknown_error_code");
        assert!(api_error.hint.is_none());
    }

    #[test]
    fn hint_covers_all_common_error_types() {
        let test_cases = [
            ("missing_path", true),
            ("path_not_found", true),
            ("invalid_extension", true),
            ("io_error", true),
            ("execution_timeout", true),
            ("execution_cancelled", true),
            ("selection_cancelled", true),
            ("shortcut_register_failed", true),
        ];

        for (code, expect_hint) in test_cases {
            let api_error = ApiError::new(code, "test");
            assert_eq!(
                api_error.hint.is_some(),
                expect_hint,
                "error code '{code}' should have hint = {expect_hint}"
            );
        }
    }
}
