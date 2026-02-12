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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunTranscriptionResponse {
    pub status: TranscriptionRunStatus,
    pub transcript: String,
}
