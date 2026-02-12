use crate::contracts::{
    AppHealthResponse, AudioPathValidationRequest, AudioPathValidationResponse,
    ModelPathValidationRequest, ModelPathValidationResponse, RunTranscriptionRequest,
    RunTranscriptionResponse, SystemCapabilityResponse, TranscriptionRunStatus,
};
use crate::errors::{ApiError, FrontendError};
use crate::execution::parse_whisper_cli_stdout;
use std::path::{Path, PathBuf};

pub type CommandResult<T> = Result<T, ApiError>;

const AUDIO_EXTENSIONS: [&str; 5] = ["wav", "mp3", "flac", "ogg", "m4a"];

pub fn app_health() -> AppHealthResponse {
    AppHealthResponse {
        status: "ok".to_owned(),
        app: "frontend-tauri".to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
    }
}

pub fn system_capability() -> SystemCapabilityResponse {
    SystemCapabilityResponse {
        os: std::env::consts::OS.to_owned(),
        arch: std::env::consts::ARCH.to_owned(),
        has_tauri_runtime: true,
    }
}

pub fn validate_model_path(
    request: ModelPathValidationRequest,
) -> CommandResult<ModelPathValidationResponse> {
    let normalized_path = normalize_path_input(&request.path)?;
    validate_model_file(&normalized_path)?;

    Ok(ModelPathValidationResponse {
        is_valid: true,
        normalized_path: normalized_path.display().to_string(),
    })
}

pub fn validate_audio_path(
    request: AudioPathValidationRequest,
) -> CommandResult<AudioPathValidationResponse> {
    let normalized_path = normalize_path_input(&request.path)?;
    validate_audio_file(&normalized_path)?;

    Ok(AudioPathValidationResponse {
        is_valid: true,
        normalized_path: normalized_path.display().to_string(),
    })
}

pub fn run_transcription_mvp(
    request: RunTranscriptionRequest,
) -> CommandResult<RunTranscriptionResponse> {
    let model = validate_model_path(ModelPathValidationRequest {
        path: request.model_path,
    })?;
    let audio = validate_audio_path(AudioPathValidationRequest {
        path: request.audio_path,
    })?;

    let model_name = Path::new(&model.normalized_path)
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("model");

    let audio_name = Path::new(&audio.normalized_path)
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("audio");

    Ok(RunTranscriptionResponse {
        status: TranscriptionRunStatus::Success,
        transcript: parse_whisper_cli_stdout(&format!(
            "[00:00.000 --> 00:01.000]  MVP transcript placeholder for {audio_name} with model {model_name}"
        )),
    })
}

fn normalize_path_input(raw_path: &str) -> Result<PathBuf, ApiError> {
    let trimmed = raw_path.trim();
    if trimmed.is_empty() {
        return Err(FrontendError::MissingPath { kind: "input" }.into());
    }

    Ok(PathBuf::from(trimmed))
}

fn validate_model_file(path: &Path) -> Result<(), ApiError> {
    if !path.exists() {
        return Err(FrontendError::NotFound {
            kind: "model",
            path: path.to_path_buf(),
        }
        .into());
    }

    if !path.is_file() {
        return Err(FrontendError::InvalidExtension {
            kind: "model",
            path: path.to_path_buf(),
            expected: "a .bin file",
        }
        .into());
    }

    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_default();

    if extension != "bin" {
        return Err(FrontendError::InvalidExtension {
            kind: "model",
            path: path.to_path_buf(),
            expected: "a .bin file",
        }
        .into());
    }

    Ok(())
}

fn validate_audio_file(path: &Path) -> Result<(), ApiError> {
    if !path.exists() {
        return Err(FrontendError::NotFound {
            kind: "audio",
            path: path.to_path_buf(),
        }
        .into());
    }

    if !path.is_file() {
        return Err(FrontendError::InvalidExtension {
            kind: "audio",
            path: path.to_path_buf(),
            expected: "an audio file",
        }
        .into());
    }

    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_default();

    if !AUDIO_EXTENSIONS.iter().any(|allowed| allowed == &extension) {
        return Err(FrontendError::InvalidExtension {
            kind: "audio",
            path: path.to_path_buf(),
            expected: "one of: wav, mp3, flac, ogg, m4a",
        }
        .into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        app_health, run_transcription_mvp, system_capability, validate_audio_path,
        validate_model_path,
    };
    use crate::contracts::{
        AudioPathValidationRequest, ModelPathValidationRequest, RunTranscriptionRequest,
        TranscriptionRunStatus,
    };
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_path(suffix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!("frontend-tauri-{nanos}.{suffix}"))
    }

    #[test]
    fn app_health_reports_ok() {
        let response = app_health();

        assert_eq!(response.status, "ok");
        assert_eq!(response.app, "frontend-tauri");
        assert!(!response.version.is_empty());
    }

    #[test]
    fn system_capability_has_os_and_arch() {
        let response = system_capability();

        assert!(!response.os.is_empty());
        assert!(!response.arch.is_empty());
        assert!(response.has_tauri_runtime);
    }

    #[test]
    fn validate_model_path_accepts_bin_file() {
        let path = unique_path("bin");
        fs::write(&path, b"model").expect("should write temp model file");

        let response = validate_model_path(ModelPathValidationRequest {
            path: path.display().to_string(),
        })
        .expect("model path should validate");

        assert!(response.is_valid);
    }

    #[test]
    fn validate_model_path_rejects_non_bin_extension() {
        let path = unique_path("txt");
        fs::write(&path, b"not a model").expect("should write temp file");

        let error = validate_model_path(ModelPathValidationRequest {
            path: path.display().to_string(),
        })
        .expect_err("model path should fail for non-bin file");

        assert_eq!(error.code, "invalid_extension");
    }

    #[test]
    fn validate_audio_path_accepts_supported_extension() {
        let path = unique_path("wav");
        fs::write(&path, b"audio").expect("should write temp audio file");

        let response = validate_audio_path(AudioPathValidationRequest {
            path: path.display().to_string(),
        })
        .expect("audio path should validate");

        assert!(response.is_valid);
    }

    #[test]
    fn validate_audio_path_rejects_unsupported_extension() {
        let path = unique_path("png");
        fs::write(&path, b"not audio").expect("should write temp file");

        let error = validate_audio_path(AudioPathValidationRequest {
            path: path.display().to_string(),
        })
        .expect_err("audio path should fail for unsupported extension");

        assert_eq!(error.code, "invalid_extension");
    }

    #[test]
    fn run_transcription_mvp_succeeds_for_valid_inputs() {
        let model_path = unique_path("bin");
        let audio_path = unique_path("wav");
        fs::write(&model_path, b"model").expect("should write temp model file");
        fs::write(&audio_path, b"audio").expect("should write temp audio file");

        let response = run_transcription_mvp(RunTranscriptionRequest {
            model_path: model_path.display().to_string(),
            audio_path: audio_path.display().to_string(),
        })
        .expect("run_transcription_mvp should succeed");

        assert_eq!(response.status, TranscriptionRunStatus::Success);
        assert!(response.transcript.contains("MVP transcript placeholder"));
    }

    #[test]
    fn run_transcription_mvp_fails_for_missing_model() {
        let missing_model = unique_path("bin");
        let audio_path = unique_path("wav");
        fs::write(&audio_path, b"audio").expect("should write temp audio file");

        let error = run_transcription_mvp(RunTranscriptionRequest {
            model_path: missing_model.display().to_string(),
            audio_path: audio_path.display().to_string(),
        })
        .expect_err("run_transcription_mvp should fail when model is missing");

        assert_eq!(error.code, "path_not_found");
    }
}
