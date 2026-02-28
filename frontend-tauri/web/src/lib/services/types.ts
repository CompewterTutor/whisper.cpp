/**
 * Type definitions for Tauri API commands and responses
 */

// Validation types
export interface ValidationResult {
	normalized_path: string;
	exists: boolean;
}

export interface ApiError {
	code: string;
	message: string;
	hint?: string;
}

export interface AppHealthResponse {
	status: string;
	version: string;
}

// Advanced options and presets
export interface AdvancedOptions {
	task?: string | null;
	language?: string | null;
	threads?: number | null;
	beam_size?: number | null;
	best_of?: number | null;
	temperature?: number | null;
}

export interface Preset {
	name: string;
	advanced: AdvancedOptions;
}

// Queue types
export type QueueItemStatus = 'Pending' | 'Running' | 'Success' | 'Error';

export interface QueueItem {
	id: string;
	audio_path: string;
	status: QueueItemStatus;
	error_message?: string;
}

export interface QueueItemStatusUpdate {
	status: QueueItemStatus;
	errorMessage?: string;
}

// History types
export interface HistoryItem {
	id: string;
	audio_path: string;
	output_path?: string;
	timestamp_ms: number;
	success: boolean;
}

// App settings types
export interface AppSettings {
	theme: string;
	default_output_dir: string;
	default_model_dir: string;
	default_threads: number | null;
	default_timeout_ms: number | null;
	diagnostics_enabled: boolean;
	start_in_background: boolean;
	launch_on_login: boolean;
}

// PTT routing types
export interface PttRouting {
	copy_to_clipboard: boolean;
	save_to_file: boolean;
	type_emulation: boolean;
}

// Shortcut types
export interface ShortcutSettings {
	enabled: boolean;
	ptt: string;
	type: string;
	clipboard: string;
	file: string;
}

// Audio device types
export interface AudioDevice {
	name: string;
	is_default: boolean;
}

// PTT capture types
export type PttState = 'idle' | 'listening' | 'transcribing' | 'error';

export interface CaptureResult {
	transcript: string;
}

// Transcription result
export interface TranscriptionResult {
	transcript: string;
}
