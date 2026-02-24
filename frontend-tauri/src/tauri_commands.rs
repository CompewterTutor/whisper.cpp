use crate::commands;
use crate::config::{ConfigStore, TranscriptionPreset};
use crate::contracts::{
    AppHealthResponse, AppSettingsResponse, AudioPathValidationResponse, BoolSettingRequest,
    ModelPathValidationResponse, PathSettingRequest, RunTranscriptionOptions,
    RunTranscriptionResponse, SystemCapabilityResponse, ThemeSettingRequest,
    TranscriptionAdvancedOptions, U16SettingRequest, U64SettingRequest,
};
use crate::errors::ApiError;
use std::path::PathBuf;
use tauri::State;
#[cfg(desktop)]
use tauri_plugin_global_shortcut::GlobalShortcutExt;

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
        advanced: None,
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
        advanced: None,
    })
}

#[tauri::command]
pub async fn export_transcript_command(
    transcript: String,
    format: String,
    app: tauri::AppHandle,
) -> TauriCommandResult<String> {
    use tauri_plugin_dialog::DialogExt;

    let artifact = commands::prepare_transcript_export(transcript, format)?;

    let file_path = app
        .dialog()
        .file()
        .add_filter(
            format!("Transcript ({})", artifact.extension).as_str(),
            &[artifact.extension.as_str()],
        )
        .set_file_name(&artifact.file_name)
        .blocking_save_file();

    let Some(path) = file_path else {
        return Err(ApiError::new(
            "selection_cancelled",
            "export destination selection cancelled",
        ));
    };

    let path_str = path.to_string();
    std::fs::write(&path_str, artifact.content).map_err(|error| {
        ApiError::new("io_error", format!("failed to write export file: {error}"))
    })?;

    Ok(path_str)
}

#[tauri::command]
pub fn open_output_folder_command(file_path: String) -> TauriCommandResult<()> {
    let trimmed = file_path.trim();
    if trimmed.is_empty() {
        return Err(ApiError::new("invalid_input", "file path cannot be empty"));
    }

    let path = PathBuf::from(trimmed);
    let parent = path
        .parent()
        .ok_or_else(|| ApiError::new("invalid_input", "file path must have a parent directory"))?;

    if !parent.exists() {
        return Err(ApiError::new(
            "path_not_found",
            format!("output directory does not exist: {}", parent.display()),
        ));
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

    command.spawn().map_err(|error| {
        ApiError::new("io_error", format!("failed to open output folder: {error}"))
    })?;

    Ok(())
}

#[tauri::command]
pub fn register_global_shortcut_command(
    app: tauri::AppHandle,
    shortcut: String,
) -> TauriCommandResult<()> {
    let trimmed = validate_shortcut_input(&shortcut)?;

    #[cfg(desktop)]
    {
        app.global_shortcut().register(trimmed).map_err(|error| {
            ApiError::new(
                "shortcut_register_failed",
                format!("failed to register shortcut '{trimmed}': {error}"),
            )
        })?;
    }

    Ok(())
}

#[tauri::command]
pub fn unregister_global_shortcut_command(
    app: tauri::AppHandle,
    shortcut: String,
) -> TauriCommandResult<()> {
    let trimmed = validate_shortcut_input(&shortcut)?;

    #[cfg(desktop)]
    {
        app.global_shortcut().unregister(trimmed).map_err(|error| {
            ApiError::new(
                "shortcut_unregister_failed",
                format!("failed to unregister shortcut '{trimmed}': {error}"),
            )
        })?;
    }

    Ok(())
}

#[tauri::command]
pub fn is_shortcut_registered_command(
    app: tauri::AppHandle,
    shortcut: String,
) -> TauriCommandResult<bool> {
    let trimmed = validate_shortcut_input(&shortcut)?;

    #[cfg(desktop)]
    {
        let is_registered = app.global_shortcut().is_registered(trimmed);
        Ok(is_registered)
    }

    #[cfg(not(desktop))]
    {
        let _ = (app, trimmed);
        Ok(false)
    }
}

fn validate_shortcut_input(shortcut: &str) -> TauriCommandResult<&str> {
    let trimmed = shortcut.trim();
    if trimmed.is_empty() {
        return Err(ApiError::new("invalid_input", "shortcut cannot be empty"));
    }

    Ok(trimmed)
}

#[tauri::command]
pub async fn pick_model_path_command(
    app: tauri::AppHandle,
) -> TauriCommandResult<String> {
    use tauri_plugin_dialog::DialogExt;

    let file_path = app
        .dialog()
        .file()
        .add_filter("Whisper Model", &["bin"])
        .blocking_pick_file();

    let Some(path) = file_path else {
        return Err(ApiError::new(
            "selection_cancelled",
            "file selection cancelled",
        ));
    };

    Ok(path.to_string())
}

#[tauri::command]
pub async fn pick_audio_path_command(
    app: tauri::AppHandle,
) -> TauriCommandResult<String> {
    use tauri_plugin_dialog::DialogExt;

    let file_path = app
        .dialog()
        .file()
        .add_filter("Audio", &["wav", "mp3", "flac", "ogg", "m4a"])
        .blocking_pick_file();

    let Some(path) = file_path else {
        return Err(ApiError::new(
            "selection_cancelled",
            "file selection cancelled",
        ));
    };

    Ok(path.to_string())
}

#[tauri::command]
pub fn get_app_settings_command(
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<AppSettingsResponse> {
    let config = config_store.load().map_err(|error| {
        ApiError::new("config_error", format!("failed to load settings: {error}"))
    })?;

    Ok(AppSettingsResponse {
        start_in_background: config.start_in_background,
        launch_on_login: config.launch_on_login,
        theme: config.theme,
        default_output_dir: config
            .default_output_dir
            .map(|p| p.to_string_lossy().into_owned()),
        default_model_dir: config
            .default_model_dir
            .map(|p| p.to_string_lossy().into_owned()),
        diagnostics_enabled: config.diagnostics_enabled,
        default_threads: config.default_threads,
        default_timeout_ms: config.default_timeout_ms,
    })
}

#[tauri::command]
pub fn set_start_in_background_command(
    enabled: bool,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    config_store
        .set_start_in_background(enabled)
        .map_err(|error| {
            ApiError::new(
                "config_error",
                format!("failed to save start_in_background setting: {error}"),
            )
        })?;
    Ok(())
}

#[tauri::command]
pub fn set_launch_on_login_command(
    app: tauri::AppHandle,
    enabled: bool,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    #[cfg(desktop)]
    {
        use tauri_plugin_autostart::ManagerExt;

        if enabled {
            app.autolaunch().enable().map_err(|error| {
                ApiError::new(
                    "autostart_error",
                    format!("failed to enable autostart: {error}"),
                )
            })?;
        } else {
            app.autolaunch().disable().map_err(|error| {
                ApiError::new(
                    "autostart_error",
                    format!("failed to disable autostart: {error}"),
                )
            })?;
        }
    }

    config_store.set_launch_on_login(enabled).map_err(|error| {
        ApiError::new(
            "config_error",
            format!("failed to save launch_on_login setting: {error}"),
        )
    })?;
    Ok(())
}

#[tauri::command]
pub fn list_presets_command(
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<Vec<TranscriptionPreset>> {
    config_store
        .list_presets()
        .map_err(|error| ApiError::new("config_error", format!("failed to list presets: {error}")))
}

#[tauri::command]
pub fn get_preset_command(
    name: String,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<Option<TranscriptionPreset>> {
    Ok(config_store.get_preset(&name))
}

#[tauri::command]
pub fn save_preset_command(
    name: String,
    advanced: TranscriptionAdvancedOptions,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    config_store.save_preset(name, advanced).map_err(|error| {
        ApiError::new("config_error", format!("failed to save preset: {error}"))
    })?;
    Ok(())
}

#[tauri::command]
pub fn delete_preset_command(
    name: String,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    config_store.delete_preset(&name).map_err(|error| {
        ApiError::new("config_error", format!("failed to delete preset: {error}"))
    })?;
    Ok(())
}

#[tauri::command]
pub fn set_default_preset_command(
    name: Option<String>,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    config_store.set_default_preset(name).map_err(|error| {
        ApiError::new(
            "config_error",
            format!("failed to set default preset: {error}"),
        )
    })?;
    Ok(())
}

#[tauri::command]
pub fn get_default_preset_command(
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<Option<String>> {
    let config = config_store.load().map_err(|error| {
        ApiError::new("config_error", format!("failed to load config: {error}"))
    })?;
    Ok(config.default_preset)
}

// Queue commands
#[tauri::command]
pub fn add_to_queue_command(
    audio_path: String,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<crate::contracts::QueueItem> {
    config_store
        .add_to_queue(audio_path)
        .map_err(|error| ApiError::new("config_error", format!("failed to add to queue: {error}")))
}

#[tauri::command]
pub fn remove_from_queue_command(
    id: String,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    config_store.remove_from_queue(&id).map_err(|error| {
        ApiError::new(
            "config_error",
            format!("failed to remove from queue: {error}"),
        )
    })
}

#[tauri::command]
pub fn reorder_queue_command(
    from_index: usize,
    to_index: usize,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    config_store
        .reorder_queue(from_index, to_index)
        .map_err(|error| ApiError::new("config_error", format!("failed to reorder queue: {error}")))
}

#[tauri::command]
pub fn get_queue_command(
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<Vec<crate::contracts::QueueItem>> {
    config_store
        .get_queue()
        .map_err(|error| ApiError::new("config_error", format!("failed to get queue: {error}")))
}

#[tauri::command]
pub fn clear_completed_queue_command(
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    config_store.clear_completed_queue_items().map_err(|error| {
        ApiError::new(
            "config_error",
            format!("failed to clear completed items: {error}"),
        )
    })
}

#[tauri::command]
pub fn update_queue_item_status_command(
    id: String,
    status: crate::contracts::QueueItemStatus,
    error_message: Option<String>,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    config_store
        .update_queue_item_status(&id, status, error_message)
        .map_err(|error| {
            ApiError::new(
                "config_error",
                format!("failed to update queue item status: {error}"),
            )
        })
}

// History commands
#[tauri::command]
pub fn add_to_history_command(
    audio_path: String,
    output_path: Option<String>,
    success: bool,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<crate::contracts::HistoryItem> {
    config_store
        .add_to_history(audio_path, output_path, success)
        .map_err(|error| {
            ApiError::new("config_error", format!("failed to add to history: {error}"))
        })
}

#[tauri::command]
pub fn get_history_command(
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<Vec<crate::contracts::HistoryItem>> {
    config_store
        .get_history()
        .map_err(|error| ApiError::new("config_error", format!("failed to get history: {error}")))
}

#[tauri::command]
pub fn clear_history_command(config_store: State<'_, ConfigStore>) -> TauriCommandResult<()> {
    config_store
        .clear_history()
        .map_err(|error| ApiError::new("config_error", format!("failed to clear history: {error}")))
}

// Clipboard commands for PTT output routing
#[tauri::command]
pub fn copy_to_clipboard_command(app: tauri::AppHandle, text: String) -> TauriCommandResult<()> {
    #[cfg(desktop)]
    {
        use tauri_plugin_clipboard_manager::ClipboardExt;

        app.clipboard().write_text(&text).map_err(|error| {
            ApiError::new(
                "clipboard_error",
                format!("failed to copy to clipboard: {error}"),
            )
        })?;
    }

    #[cfg(not(desktop))]
    {
        let _ = (app, text);
    }

    Ok(())
}

#[tauri::command]
pub fn get_clipboard_text_command(app: tauri::AppHandle) -> TauriCommandResult<String> {
    #[cfg(desktop)]
    {
        use tauri_plugin_clipboard_manager::ClipboardExt;

        let text = app.clipboard().read_text().map_err(|error| {
            ApiError::new(
                "clipboard_error",
                format!("failed to read clipboard: {error}"),
            )
        })?;

        Ok(text)
    }

    #[cfg(not(desktop))]
    {
        let _ = app;
        Ok(String::new())
    }
}

// PTT output routing settings
#[tauri::command]
pub fn get_ptt_routing_command(
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<crate::contracts::PttOutputRouting> {
    let config = config_store.load().map_err(|error| {
        ApiError::new("config_error", format!("failed to load config: {error}"))
    })?;

    Ok(config
        .ptt_routing
        .unwrap_or_else(crate::contracts::PttOutputRouting::default))
}

#[tauri::command]
pub fn set_ptt_routing_command(
    routing: crate::contracts::PttOutputRouting,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    config_store.set_ptt_routing(routing).map_err(|error| {
        ApiError::new(
            "config_error",
            format!("failed to save PTT routing: {error}"),
        )
    })
}

// P4: Additional settings commands
#[tauri::command]
pub fn set_theme_command(
    request: ThemeSettingRequest,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    let theme = request.theme.trim();
    if !["system", "light", "dark"].contains(&theme) {
        return Err(ApiError::new(
            "invalid_input",
            "theme must be 'system', 'light', or 'dark'",
        ));
    }
    config_store
        .set_theme(theme.to_owned())
        .map_err(|error| ApiError::new("config_error", format!("failed to save theme: {error}")))?;
    Ok(())
}

#[tauri::command]
pub fn set_default_output_dir_command(
    request: PathSettingRequest,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    let path = if request.path.trim().is_empty() {
        None
    } else {
        Some(PathBuf::from(request.path.trim()))
    };
    config_store.set_default_output_dir(path).map_err(|error| {
        ApiError::new(
            "config_error",
            format!("failed to save default output directory: {error}"),
        )
    })?;
    Ok(())
}

#[tauri::command]
pub fn set_default_model_dir_command(
    request: PathSettingRequest,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    let path = if request.path.trim().is_empty() {
        None
    } else {
        Some(PathBuf::from(request.path.trim()))
    };
    config_store.set_default_model_dir(path).map_err(|error| {
        ApiError::new(
            "config_error",
            format!("failed to save default model directory: {error}"),
        )
    })?;
    Ok(())
}

#[tauri::command]
pub fn set_diagnostics_enabled_command(
    request: BoolSettingRequest,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    config_store
        .set_diagnostics_enabled(request.enabled)
        .map_err(|error| {
            ApiError::new(
                "config_error",
                format!("failed to save diagnostics setting: {error}"),
            )
        })?;
    Ok(())
}

#[tauri::command]
pub fn set_default_threads_command(
    request: U16SettingRequest,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    if let Some(threads) = request.value
        && (threads == 0 || threads > 128)
    {
        return Err(ApiError::new(
            "invalid_input",
            "threads must be between 1 and 128",
        ));
    }
    config_store
        .set_default_threads(request.value)
        .map_err(|error| {
            ApiError::new(
                "config_error",
                format!("failed to save default threads: {error}"),
            )
        })?;
    Ok(())
}

#[tauri::command]
pub fn set_default_timeout_command(
    request: U64SettingRequest,
    config_store: State<'_, ConfigStore>,
) -> TauriCommandResult<()> {
    if let Some(timeout) = request.value
        && timeout < 1000
    {
        return Err(ApiError::new(
            "invalid_input",
            "timeout must be at least 1000ms",
        ));
    }
    config_store
        .set_default_timeout_ms(request.value)
        .map_err(|error| {
            ApiError::new(
                "config_error",
                format!("failed to save default timeout: {error}"),
            )
        })?;
    Ok(())
}

#[tauri::command]
pub async fn pick_directory_command(
    app: tauri::AppHandle,
) -> TauriCommandResult<String> {
    use tauri_plugin_dialog::DialogExt;

    let folder_path = app.dialog().file().blocking_pick_folder();

    let Some(path) = folder_path else {
        return Err(ApiError::new(
            "selection_cancelled",
            "directory selection cancelled",
        ));
    };

    Ok(path.to_string())
}

// P6: Audio capture commands
use crate::audio::{AudioCaptureSession, AudioDeviceInfo, CaptureState};
use std::sync::Mutex;

#[tauri::command]
pub fn list_audio_devices_command(
    capture_session: State<'_, Mutex<AudioCaptureSession>>,
) -> TauriCommandResult<Vec<AudioDeviceInfo>> {
    let session = capture_session
        .lock()
        .map_err(|_| ApiError::new("lock_error", "failed to acquire audio session lock"))?;
    session
        .list_input_devices()
        .map_err(|e| ApiError::new("audio_error", format!("failed to list audio devices: {e}")))
}

#[tauri::command]
pub fn select_audio_device_command(
    device_name: Option<String>,
    capture_session: State<'_, Mutex<AudioCaptureSession>>,
) -> TauriCommandResult<()> {
    let mut session = capture_session
        .lock()
        .map_err(|_| ApiError::new("lock_error", "failed to acquire audio session lock"))?;
    session
        .select_device(device_name.as_deref())
        .map_err(|e| ApiError::new("audio_error", format!("failed to select audio device: {e}")))
}

#[tauri::command]
pub fn get_capture_state_command(
    capture_session: State<'_, Mutex<AudioCaptureSession>>,
) -> TauriCommandResult<CaptureState> {
    let session = capture_session
        .lock()
        .map_err(|_| ApiError::new("lock_error", "failed to acquire audio session lock"))?;
    Ok(session.state())
}

#[tauri::command]
pub fn start_capture_command(
    capture_session: State<'_, Mutex<AudioCaptureSession>>,
) -> TauriCommandResult<()> {
    let mut session = capture_session
        .lock()
        .map_err(|_| ApiError::new("lock_error", "failed to acquire audio session lock"))?;
    session
        .start_capture()
        .map_err(|e| ApiError::new("audio_error", format!("failed to start capture: {e}")))
}

#[tauri::command]
pub fn stop_capture_command(
    capture_session: State<'_, Mutex<AudioCaptureSession>>,
) -> TauriCommandResult<()> {
    let mut session = capture_session
        .lock()
        .map_err(|_| ApiError::new("lock_error", "failed to acquire audio session lock"))?;
    session
        .stop_capture()
        .map_err(|e| ApiError::new("audio_error", format!("failed to stop capture: {e}")))?;
    Ok(())
}

#[tauri::command]
pub fn get_current_audio_device_command(
    capture_session: State<'_, Mutex<AudioCaptureSession>>,
) -> TauriCommandResult<Option<AudioDeviceInfo>> {
    let session = capture_session
        .lock()
        .map_err(|_| ApiError::new("lock_error", "failed to acquire audio session lock"))?;
    Ok(session.current_device_info())
}

#[cfg(test)]
mod tests {
    use super::{
        app_health_command, export_transcript_command, open_output_folder_command,
        run_transcription_command, run_transcription_with_options_command,
        system_capability_command, validate_audio_path_command, validate_model_path_command,
        validate_shortcut_input,
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

    #[test]
    fn validate_shortcut_input_rejects_empty_shortcut() {
        let error = validate_shortcut_input("   ").expect_err("empty shortcut should be rejected");

        assert_eq!(error.code, "invalid_input");
    }

    #[test]
    fn validate_shortcut_input_accepts_valid_shortcut() {
        let shortcut = validate_shortcut_input("CmdOrCtrl+Shift+Space")
            .expect("valid shortcut should be accepted");

        assert_eq!(shortcut, "CmdOrCtrl+Shift+Space");
    }
}
