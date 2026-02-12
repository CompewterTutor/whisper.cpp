use crate::commands;
use crate::contracts::{
    AppHealthResponse, AudioPathValidationResponse, ModelPathValidationResponse,
    RunTranscriptionOptions, RunTranscriptionResponse, SystemCapabilityResponse,
};
use crate::errors::ApiError;
use std::path::PathBuf;

pub type TauriCommandResult<T> = Result<T, ApiError>;

#[tauri::command]
pub fn app_health_command() -> TauriCommandResult<AppHealthResponse> {
    Ok(commands::app_health())
}

#[tauri::command]
pub fn system_capability_command() -> TauriCommandResult<SystemCapabilityResponse> {
    Ok(commands::system_capability())
}

#[tauri::command]
pub fn validate_model_path_command(
    path: String,
) -> TauriCommandResult<ModelPathValidationResponse> {
    commands::validate_model_path(crate::contracts::ModelPathValidationRequest { path })
}

#[tauri::command]
pub fn validate_audio_path_command(
    path: String,
) -> TauriCommandResult<AudioPathValidationResponse> {
    commands::validate_audio_path(crate::contracts::AudioPathValidationRequest { path })
}

#[tauri::command]
pub fn run_transcription_command(
    model_path: String,
    audio_path: String,
) -> TauriCommandResult<RunTranscriptionResponse> {
    commands::run_transcription_mvp(crate::contracts::RunTranscriptionRequest {
        model_path,
        audio_path,
        options: None,
    })
}

#[tauri::command]
pub fn run_transcription_with_options_command(
    model_path: String,
    audio_path: String,
    timeout_ms: Option<u64>,
    cancel_requested: bool,
) -> TauriCommandResult<RunTranscriptionResponse> {
    commands::run_transcription_with_execution(crate::contracts::RunTranscriptionRequest {
        model_path,
        audio_path,
        options: Some(RunTranscriptionOptions {
            timeout_ms,
            cancel_requested,
        }),
    })
}

#[tauri::command]
pub fn export_transcript_command(transcript: String, format: String) -> TauriCommandResult<String> {
    let artifact = commands::prepare_transcript_export(transcript, format)?;

    let Some(path) = rfd::FileDialog::new()
        .add_filter(
            format!("Transcript ({})", artifact.extension).as_str(),
            &[artifact.extension.as_str()],
        )
        .set_file_name(&artifact.file_name)
        .save_file()
    else {
        return Err(ApiError {
            code: "selection_cancelled".to_owned(),
            message: "export destination selection cancelled".to_owned(),
        });
    };

    std::fs::write(&path, artifact.content).map_err(|error| ApiError {
        code: "io_error".to_owned(),
        message: format!("failed to write export file: {error}"),
    })?;

    Ok(path.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn open_output_folder_command(file_path: String) -> TauriCommandResult<()> {
    let trimmed = file_path.trim();
    if trimmed.is_empty() {
        return Err(ApiError {
            code: "invalid_input".to_owned(),
            message: "file path cannot be empty".to_owned(),
        });
    }

    let path = PathBuf::from(trimmed);
    let parent = path.parent().ok_or_else(|| ApiError {
        code: "invalid_input".to_owned(),
        message: "file path must have a parent directory".to_owned(),
    })?;

    if !parent.exists() {
        return Err(ApiError {
            code: "path_not_found".to_owned(),
            message: format!("output directory does not exist: {}", parent.display()),
        });
    }

    #[cfg(target_os = "windows")]
    let mut command = {
        let mut cmd = std::process::Command::new("explorer");
        cmd.arg(parent);
        cmd
    };

    #[cfg(target_os = "macos")]
    let mut command = {
        let mut cmd = std::process::Command::new("open");
        cmd.arg(parent);
        cmd
    };

    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = {
        let mut cmd = std::process::Command::new("xdg-open");
        cmd.arg(parent);
        cmd
    };

    command.spawn().map_err(|error| ApiError {
        code: "io_error".to_owned(),
        message: format!("failed to open output folder: {error}"),
    })?;

    Ok(())
}

#[tauri::command]
pub fn pick_model_path_command() -> TauriCommandResult<String> {
    pick_file_with_filter(&[("Whisper Model", &["bin"])])
}

#[tauri::command]
pub fn pick_audio_path_command() -> TauriCommandResult<String> {
    pick_file_with_filter(&[("Audio", &["wav", "mp3", "flac", "ogg", "m4a"])])
}

fn pick_file_with_filter(filters: &[(&str, &[&str])]) -> TauriCommandResult<String> {
    let mut dialog = rfd::FileDialog::new();
    for &(name, extensions) in filters {
        dialog = dialog.add_filter(name, extensions);
    }

    let Some(path) = dialog.pick_file() else {
        return Err(ApiError {
            code: "selection_cancelled".to_owned(),
            message: "file selection cancelled".to_owned(),
        });
    };

    Ok(path.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::{
        app_health_command, export_transcript_command, open_output_folder_command,
        run_transcription_command, run_transcription_with_options_command,
        system_capability_command, validate_audio_path_command, validate_model_path_command,
    };
    use crate::contracts::TranscriptionRunStatus;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_path(suffix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!("frontend-tauri-commands-{nanos}.{suffix}"))
    }

    #[test]
    fn app_health_command_returns_ok() {
        let response = app_health_command().expect("app_health_command should succeed");
        assert_eq!(response.status, "ok");
    }

    #[test]
    fn system_capability_command_returns_runtime_info() {
        let response =
            system_capability_command().expect("system_capability_command should succeed");
        assert!(!response.os.is_empty());
        assert!(response.has_tauri_runtime);
    }

    #[test]
    fn validate_commands_accept_valid_paths() {
        let model = unique_path("bin");
        let audio = unique_path("wav");
        fs::write(&model, b"model").expect("should write model file");
        fs::write(&audio, b"audio").expect("should write audio file");

        let model_response = validate_model_path_command(model.display().to_string())
            .expect("model validation should succeed");
        let audio_response = validate_audio_path_command(audio.display().to_string())
            .expect("audio validation should succeed");

        assert!(model_response.is_valid);
        assert!(audio_response.is_valid);
    }

    #[test]
    fn run_transcription_command_returns_success() {
        let model = unique_path("bin");
        let audio = unique_path("wav");
        fs::write(&model, b"model").expect("should write model file");
        fs::write(&audio, b"audio").expect("should write audio file");

        let response =
            run_transcription_command(model.display().to_string(), audio.display().to_string())
                .expect("run_transcription_command should succeed");

        assert_eq!(response.status, TranscriptionRunStatus::Success);
    }

    #[test]
    fn run_transcription_with_options_command_can_cancel() {
        let model = unique_path("bin");
        let audio = unique_path("wav");
        fs::write(&model, b"model").expect("should write model file");
        fs::write(&audio, b"audio").expect("should write audio file");

        let error = run_transcription_with_options_command(
            model.display().to_string(),
            audio.display().to_string(),
            Some(1_000),
            true,
        )
        .expect_err("cancelled command should fail");

        assert_eq!(error.code, "execution_cancelled");
    }

    #[test]
    fn export_transcript_command_rejects_invalid_format() {
        let error = export_transcript_command("hello".to_owned(), "xml".to_owned())
            .expect_err("invalid format should fail before dialog");

        assert_eq!(error.code, "invalid_input");
    }

    #[test]
    fn open_output_folder_command_rejects_empty_path() {
        let error = open_output_folder_command("   ".to_owned())
            .expect_err("empty path should be rejected");

        assert_eq!(error.code, "invalid_input");
    }
}
