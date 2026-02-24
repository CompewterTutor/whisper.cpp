use crate::contracts::{
    AppHealthResponse, AudioPathValidationRequest, AudioPathValidationResponse,
    ModelPathValidationRequest, ModelPathValidationResponse, RunTranscriptionRequest,
    RunTranscriptionResponse, SystemCapabilityResponse, TranscriptionRunStatus,
};
use crate::errors::{ApiError, FrontendError};
use crate::execution::{
    CliRunOptions, ProcessCliRunner, WhisperCliRequest, parse_whisper_cli_stdout, run_with_runner,
};
use serde::Serialize;
use std::path::{Path, PathBuf};

pub type CommandResult<T> = Result<T, ApiError>;

const AUDIO_EXTENSIONS: [&str; 5] = ["wav", "mp3", "flac", "ogg", "m4a"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranscriptExportArtifact {
    pub file_name: String,
    pub extension: String,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TranscriptExportFormat {
    Txt,
    Srt,
    Vtt,
    Json,
}

impl TranscriptExportFormat {
    fn parse(raw: &str) -> CommandResult<Self> {
        let normalized = raw.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "txt" => Ok(Self::Txt),
            "srt" => Ok(Self::Srt),
            "vtt" => Ok(Self::Vtt),
            "json" => Ok(Self::Json),
            _ => Err(ApiError::new(
                "invalid_input",
                format!("unsupported export format: {raw} (expected txt|srt|vtt|json)"),
            )),
        }
    }

    fn extension(self) -> &'static str {
        match self {
            Self::Txt => "txt",
            Self::Srt => "srt",
            Self::Vtt => "vtt",
            Self::Json => "json",
        }
    }
}

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

pub fn run_transcription_with_execution(
    request: RunTranscriptionRequest,
) -> CommandResult<RunTranscriptionResponse> {
    let model = validate_model_path(ModelPathValidationRequest {
        path: request.model_path,
    })?;
    let audio = validate_audio_path(AudioPathValidationRequest {
        path: request.audio_path,
    })?;

    let options = request.options.unwrap_or_default();
    let execution_options = CliRunOptions {
        timeout_ms: options.timeout_ms,
        cancel_requested: options.cancel_requested,
    };

    let output = run_with_runner(
        Path::new("build/bin/whisper-cli"),
        &WhisperCliRequest {
            model_path: PathBuf::from(model.normalized_path),
            audio_path: PathBuf::from(audio.normalized_path),
            advanced: request.advanced,
        },
        &ProcessCliRunner,
        execution_options,
    )
    .map_err(ApiError::from)?;

    Ok(RunTranscriptionResponse {
        status: TranscriptionRunStatus::Success,
        transcript: output.transcript,
    })
}

pub fn prepare_transcript_export(
    transcript: String,
    format: String,
) -> CommandResult<TranscriptExportArtifact> {
    let trimmed = transcript.trim();
    if trimmed.is_empty() {
        return Err(ApiError::new("invalid_input", "transcript cannot be empty"));
    }

    let export_format = TranscriptExportFormat::parse(&format)?;
    let lines: Vec<&str> = trimmed
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    let content = match export_format {
        TranscriptExportFormat::Txt => lines.join("\n"),
        TranscriptExportFormat::Srt => render_as_srt(&lines),
        TranscriptExportFormat::Vtt => render_as_vtt(&lines),
        TranscriptExportFormat::Json => render_as_json(trimmed, &lines)?,
    };

    Ok(TranscriptExportArtifact {
        file_name: format!("transcript.{}", export_format.extension()),
        extension: export_format.extension().to_owned(),
        content,
    })
}

fn render_as_srt(lines: &[&str]) -> String {
    if lines.is_empty() {
        return String::new();
    }

    lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let start_ms = index as u64 * 2_000;
            let end_ms = start_ms + 2_000;
            format!(
                "{}\n{} --> {}\n{}",
                index + 1,
                format_timestamp_srt(start_ms),
                format_timestamp_srt(end_ms),
                line
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}

fn render_as_vtt(lines: &[&str]) -> String {
    if lines.is_empty() {
        return "WEBVTT\n".to_owned();
    }

    let body = lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let start_ms = index as u64 * 2_000;
            let end_ms = start_ms + 2_000;
            format!(
                "{} --> {}\n{}",
                format_timestamp_vtt(start_ms),
                format_timestamp_vtt(end_ms),
                line
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n");

    format!("WEBVTT\n\n{body}")
}

fn render_as_json(full_text: &str, lines: &[&str]) -> CommandResult<String> {
    #[derive(Serialize)]
    struct Segment {
        index: usize,
        start_ms: u64,
        end_ms: u64,
        text: String,
    }

    #[derive(Serialize)]
    struct ExportModel {
        text: String,
        segments: Vec<Segment>,
    }

    let segments = lines
        .iter()
        .enumerate()
        .map(|(index, line)| Segment {
            index: index + 1,
            start_ms: index as u64 * 2_000,
            end_ms: (index as u64 + 1) * 2_000,
            text: (*line).to_owned(),
        })
        .collect();

    serde_json::to_string_pretty(&ExportModel {
        text: full_text.to_owned(),
        segments,
    })
    .map_err(|error| {
        ApiError::new(
            "serialization_error",
            format!("failed to serialize transcript export: {error}"),
        )
    })
}

fn format_timestamp_srt(total_ms: u64) -> String {
    let hours = total_ms / 3_600_000;
    let minutes = (total_ms % 3_600_000) / 60_000;
    let seconds = (total_ms % 60_000) / 1_000;
    let millis = total_ms % 1_000;
    format!("{hours:02}:{minutes:02}:{seconds:02},{millis:03}")
}

fn format_timestamp_vtt(total_ms: u64) -> String {
    let hours = total_ms / 3_600_000;
    let minutes = (total_ms % 3_600_000) / 60_000;
    let seconds = (total_ms % 60_000) / 1_000;
    let millis = total_ms % 1_000;
    format!("{hours:02}:{minutes:02}:{seconds:02}.{millis:03}")
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
        app_health, prepare_transcript_export, run_transcription_mvp,
        run_transcription_with_execution, system_capability, validate_audio_path,
        validate_model_path,
    };
    use crate::contracts::{
        AudioPathValidationRequest, ModelPathValidationRequest, RunTranscriptionOptions,
        RunTranscriptionRequest, TranscriptionRunStatus,
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
            options: None,
            advanced: None,
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
            options: None,
            advanced: None,
        })
        .expect_err("run_transcription_mvp should fail when model is missing");

        assert_eq!(error.code, "path_not_found");
    }

    #[test]
    fn run_transcription_with_execution_returns_cancelled_error() {
        let model_path = unique_path("bin");
        let audio_path = unique_path("wav");
        fs::write(&model_path, b"model").expect("should write temp model file");
        fs::write(&audio_path, b"audio").expect("should write temp audio file");

        let error = run_transcription_with_execution(RunTranscriptionRequest {
            model_path: model_path.display().to_string(),
            audio_path: audio_path.display().to_string(),
            options: Some(RunTranscriptionOptions {
                timeout_ms: None,
                cancel_requested: true,
            }),
            advanced: None,
        })
        .expect_err("execution run should return cancellation error");

        assert_eq!(error.code, "execution_cancelled");
    }

    #[test]
    fn prepare_transcript_export_rejects_empty_transcript() {
        let error = prepare_transcript_export("   ".to_owned(), "txt".to_owned())
            .expect_err("empty transcript should fail");

        assert_eq!(error.code, "invalid_input");
    }

    #[test]
    fn prepare_transcript_export_renders_srt() {
        let artifact = prepare_transcript_export("hello\nworld".to_owned(), "srt".to_owned())
            .expect("srt export should succeed");

        assert_eq!(artifact.extension, "srt");
        assert!(
            artifact
                .content
                .contains("1\n00:00:00,000 --> 00:00:02,000\nhello")
        );
        assert!(
            artifact
                .content
                .contains("2\n00:00:02,000 --> 00:00:04,000\nworld")
        );
    }

    #[test]
    fn prepare_transcript_export_renders_json() {
        let artifact = prepare_transcript_export("hello".to_owned(), "json".to_owned())
            .expect("json export should succeed");

        assert_eq!(artifact.extension, "json");
        assert!(artifact.content.contains("\"text\": \"hello\""));
        assert!(artifact.content.contains("\"segments\""));
    }
}
