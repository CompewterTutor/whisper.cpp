use crate::commands::CommandResult;
use crate::contracts::{RunTranscriptionRequest, RunTranscriptionResponse};
use crate::errors::ApiError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiWorkflowState {
    Idle,
    Loading,
    Success,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MvpUiState {
    pub model_path: String,
    pub audio_path: String,
    pub transcript: String,
    pub state: UiWorkflowState,
    pub error: Option<ApiError>,
}

impl Default for MvpUiState {
    fn default() -> Self {
        Self {
            model_path: String::new(),
            audio_path: String::new(),
            transcript: String::new(),
            state: UiWorkflowState::Idle,
            error: None,
        }
    }
}

impl MvpUiState {
    pub fn set_model_path(&mut self, path: impl Into<String>) {
        self.model_path = path.into();
        if matches!(self.state, UiWorkflowState::Error) {
            self.state = UiWorkflowState::Idle;
            self.error = None;
        }
    }

    pub fn set_audio_path(&mut self, path: impl Into<String>) {
        self.audio_path = path.into();
        if matches!(self.state, UiWorkflowState::Error) {
            self.state = UiWorkflowState::Idle;
            self.error = None;
        }
    }

    pub fn can_start_transcription(&self) -> bool {
        !self.model_path.trim().is_empty() && !self.audio_path.trim().is_empty()
    }

    pub fn transcript_panel_text(&self) -> &str {
        if self.transcript.is_empty() {
            "Transcript output will appear here."
        } else {
            &self.transcript
        }
    }

    pub fn error_banner_text(&self) -> Option<&str> {
        self.error.as_ref().map(|value| value.message.as_str())
    }

    pub fn run_with<Runner>(&mut self, runner: Runner)
    where
        Runner: FnOnce(RunTranscriptionRequest) -> CommandResult<RunTranscriptionResponse>,
    {
        self.state = UiWorkflowState::Loading;
        self.error = None;

        let request = RunTranscriptionRequest {
            model_path: self.model_path.clone(),
            audio_path: self.audio_path.clone(),
            options: None,
        };

        match runner(request) {
            Ok(response) => {
                self.transcript = response.transcript;
                self.state = UiWorkflowState::Success;
            }
            Err(error) => {
                self.transcript.clear();
                self.error = Some(error);
                self.state = UiWorkflowState::Error;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MvpUiState, UiWorkflowState};
    use crate::commands::run_transcription_mvp;
    use crate::errors::ApiError;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_path(suffix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!("frontend-tauri-ui-state-{nanos}.{suffix}"))
    }

    #[test]
    fn run_with_sets_success_state_and_transcript() {
        let model_path = unique_path("bin");
        let audio_path = unique_path("wav");
        fs::write(&model_path, b"model").expect("should write temp model file");
        fs::write(&audio_path, b"audio").expect("should write temp audio file");

        let mut state = MvpUiState {
            model_path: model_path.display().to_string(),
            audio_path: audio_path.display().to_string(),
            ..MvpUiState::default()
        };

        state.run_with(run_transcription_mvp);

        assert_eq!(state.state, UiWorkflowState::Success);
        assert!(state.error.is_none());
        assert!(state.transcript.contains("MVP transcript placeholder"));
    }

    #[test]
    fn run_with_sets_error_state_for_failure() {
        let mut state = MvpUiState {
            model_path: String::new(),
            audio_path: String::new(),
            ..MvpUiState::default()
        };

        state.run_with(|_| Err(ApiError::new("missing_path", "input path cannot be empty")));

        assert_eq!(state.state, UiWorkflowState::Error);
        assert_eq!(
            state.error.as_ref().map(|value| value.code.as_str()),
            Some("missing_path")
        );
        assert!(state.transcript.is_empty());
    }

    #[test]
    fn pickers_enable_start_only_when_both_paths_present() {
        let mut state = MvpUiState::default();
        assert!(!state.can_start_transcription());

        state.set_model_path("models/ggml-base.en.bin");
        assert!(!state.can_start_transcription());

        state.set_audio_path("samples/jfk.wav");
        assert!(state.can_start_transcription());
    }

    #[test]
    fn transcript_panel_returns_placeholder_when_empty() {
        let state = MvpUiState::default();
        assert_eq!(
            state.transcript_panel_text(),
            "Transcript output will appear here."
        );
    }

    #[test]
    fn integration_select_inputs_run_and_render_transcript() {
        let model_path = unique_path("bin");
        let audio_path = unique_path("wav");
        fs::write(&model_path, b"model").expect("should write temp model file");
        fs::write(&audio_path, b"audio").expect("should write temp audio file");

        let mut state = MvpUiState::default();
        state.set_model_path(model_path.display().to_string());
        state.set_audio_path(audio_path.display().to_string());

        assert!(state.can_start_transcription());

        state.run_with(run_transcription_mvp);

        assert_eq!(state.state, UiWorkflowState::Success);
        assert!(state.error_banner_text().is_none());
        assert!(
            state
                .transcript_panel_text()
                .contains("MVP transcript placeholder")
        );
    }
}
