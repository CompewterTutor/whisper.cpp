use crate::commands::run_transcription_mvp;
use crate::ui_state::{MvpUiState, UiWorkflowState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MvpAction {
    SetModelPath(String),
    SetAudioPath(String),
    StartTranscription,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MvpViewModel {
    pub model_path: String,
    pub audio_path: String,
    pub transcript_panel_text: String,
    pub error_banner_text: Option<String>,
    pub can_start: bool,
    pub state: UiWorkflowState,
}

impl From<&MvpUiState> for MvpViewModel {
    fn from(value: &MvpUiState) -> Self {
        Self {
            model_path: value.model_path.clone(),
            audio_path: value.audio_path.clone(),
            transcript_panel_text: value.transcript_panel_text().to_owned(),
            error_banner_text: value.error_banner_text().map(ToOwned::to_owned),
            can_start: value.can_start_transcription(),
            state: value.state.clone(),
        }
    }
}

pub fn dispatch_action(state: &mut MvpUiState, action: MvpAction) {
    match action {
        MvpAction::SetModelPath(path) => state.set_model_path(path),
        MvpAction::SetAudioPath(path) => state.set_audio_path(path),
        MvpAction::StartTranscription => state.run_with(run_transcription_mvp),
    }
}

#[cfg(test)]
mod tests {
    use super::{MvpAction, MvpViewModel, dispatch_action};
    use crate::ui_state::{MvpUiState, UiWorkflowState};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_path(suffix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!("frontend-tauri-binding-{nanos}.{suffix}"))
    }

    #[test]
    fn dispatch_updates_picker_fields() {
        let mut state = MvpUiState::default();

        dispatch_action(
            &mut state,
            MvpAction::SetModelPath("models/ggml-base.en.bin".to_owned()),
        );
        dispatch_action(
            &mut state,
            MvpAction::SetAudioPath("samples/jfk.wav".to_owned()),
        );

        let view = MvpViewModel::from(&state);
        assert_eq!(view.model_path, "models/ggml-base.en.bin");
        assert_eq!(view.audio_path, "samples/jfk.wav");
        assert!(view.can_start);
    }

    #[test]
    fn dispatch_start_transcription_transitions_to_success() {
        let model_path = unique_path("bin");
        let audio_path = unique_path("wav");
        fs::write(&model_path, b"model").expect("should write model file");
        fs::write(&audio_path, b"audio").expect("should write audio file");

        let mut state = MvpUiState::default();
        dispatch_action(
            &mut state,
            MvpAction::SetModelPath(model_path.display().to_string()),
        );
        dispatch_action(
            &mut state,
            MvpAction::SetAudioPath(audio_path.display().to_string()),
        );
        dispatch_action(&mut state, MvpAction::StartTranscription);

        let view = MvpViewModel::from(&state);
        assert_eq!(view.state, UiWorkflowState::Success);
        assert!(view.error_banner_text.is_none());
        assert!(
            view.transcript_panel_text
                .contains("MVP transcript placeholder")
        );
    }

    #[test]
    fn dispatch_start_transcription_transitions_to_error() {
        let mut state = MvpUiState::default();
        dispatch_action(&mut state, MvpAction::StartTranscription);

        let view = MvpViewModel::from(&state);
        assert_eq!(view.state, UiWorkflowState::Error);
        assert!(view.error_banner_text.is_some());
        assert_eq!(
            view.transcript_panel_text,
            "Transcript output will appear here."
        );
    }
}
