/**
 * Tauri API service layer - type-safe wrappers for Tauri commands
 */

import { invoke } from '@tauri-apps/api/core';

// Types for API responses
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

/**
 * Wait for Tauri API to be available
 */
export async function waitForTauri(timeoutMs = 5000): Promise<void> {
	return new Promise((resolve, reject) => {
		const startTime = Date.now();

		function check() {
			if (window.__TAURI__) {
				resolve();
				return;
			}

			if (Date.now() - startTime > timeoutMs) {
				reject(new Error('Tauri API not available after timeout'));
				return;
			}

			setTimeout(check, 50);
		}

		check();
	});
}

/**
 * Type-safe invoke wrapper with error handling
 */
export async function tauriInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
	await waitForTauri();
	return invoke<T>(command, args);
}

/**
 * Validate a model file path
 */
export async function validateModelPath(path: string): Promise<ValidationResult> {
	return tauriInvoke<ValidationResult>('validate_model_path_command', { path });
}

/**
 * Validate an audio file path
 */
export async function validateAudioPath(path: string): Promise<ValidationResult> {
	return tauriInvoke<ValidationResult>('validate_audio_path_command', { path });
}

/**
 * Open file picker for model selection
 */
export async function pickModelPath(): Promise<string> {
	return tauriInvoke<string>('pick_model_path_command');
}

/**
 * Open file picker for audio selection
 */
export async function pickAudioPath(): Promise<string> {
	return tauriInvoke<string>('pick_audio_path_command');
}

/**
 * Check app health
 */
export async function getAppHealth(): Promise<AppHealthResponse> {
	return tauriInvoke<AppHealthResponse>('app_health_command');
}

/**
 * Run transcription with model and audio paths
 */
export async function runTranscription(modelPath: string, audioPath: string): Promise<{ transcript: string }> {
	return tauriInvoke<{ transcript: string }>('run_transcription_command', {
		modelPath,
		audioPath
	});
}

/**
 * Export transcript to file
 */
export async function exportTranscript(transcript: string, format: string): Promise<string> {
	return tauriInvoke<string>('export_transcript_command', { transcript, format });
}

/**
 * Open output folder in file manager
 */
export async function openOutputFolder(filePath: string): Promise<void> {
	return tauriInvoke<void>('open_output_folder_command', { filePath });
}

/**
 * Copy text to clipboard
 */
export async function copyToClipboard(text: string): Promise<void> {
	return tauriInvoke<void>('copy_to_clipboard_command', { text });
}
