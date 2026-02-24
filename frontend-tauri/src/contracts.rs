use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppHealthResponse {
    pub status: String,
    pub app: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemCapabilityResponse {
    pub os: String,
    pub arch: String,
    pub has_tauri_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelPathValidationRequest {
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelPathValidationResponse {
    pub is_valid: bool,
    pub normalized_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioPathValidationRequest {
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioPathValidationResponse {
    pub is_valid: bool,
    pub normalized_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TranscriptionRunStatus {
    Loading,
    Success,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunTranscriptionRequest {
    pub model_path: String,
    pub audio_path: String,
    pub options: Option<RunTranscriptionOptions>,
    pub advanced: Option<TranscriptionAdvancedOptions>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RunTranscriptionOptions {
    pub timeout_ms: Option<u64>,
    pub cancel_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TranscriptionAdvancedOptions {
    /// Task type: "transcribe" or "translate"
    pub task: Option<String>,
    /// Language code (e.g., "en", "fr", "auto" for auto-detect)
    pub language: Option<String>,
    /// Number of threads to use (default: system default)
    pub threads: Option<u16>,
    /// Beam size for decoding
    pub beam_size: Option<u8>,
    /// Best-of candidates
    pub best_of: Option<u8>,
    /// Temperature for sampling (stored as integer * 100 to avoid f32 comparison issues)
    pub temperature: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunTranscriptionResponse {
    pub status: TranscriptionRunStatus,
    pub transcript: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppSettingsResponse {
    pub start_in_background: bool,
    pub launch_on_login: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetSettingRequest {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueueItemStatus {
    Pending,
    Running,
    Success,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueItem {
    pub id: String,
    pub audio_path: String,
    pub status: QueueItemStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryItem {
    pub id: String,
    pub timestamp_ms: u64,
    pub audio_path: String,
    pub output_path: Option<String>,
    pub success: bool,
}
