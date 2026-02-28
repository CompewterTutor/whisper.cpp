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

// Advanced options types
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

/**
 * List all saved presets
 */
export async function listPresets(): Promise<Preset[]> {
	return tauriInvoke<Preset[]>('list_presets_command');
}

/**
 * Get a specific preset by name
 */
export async function getPreset(name: string): Promise<Preset | null> {
	return tauriInvoke<Preset | null>('get_preset_command', { name });
}

/**
 * Save a new preset
 */
export async function savePreset(name: string, advanced: AdvancedOptions): Promise<void> {
	return tauriInvoke<void>('save_preset_command', { name, advanced });
}

/**
 * Delete a preset
 */
export async function deletePreset(name: string): Promise<void> {
	return tauriInvoke<void>('delete_preset_command', { name });
}

/**
 * Set the default preset
 */
export async function setDefaultPreset(name: string | null): Promise<void> {
	return tauriInvoke<void>('set_default_preset_command', { name });
}

/**
 * Get the default preset name
 */
export async function getDefaultPreset(): Promise<string | null> {
	return tauriInvoke<string | null>('get_default_preset_command');
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

/**
 * Get all items in the queue
 */
export async function getQueue(): Promise<QueueItem[]> {
	return tauriInvoke<QueueItem[]>('get_queue_command');
}

/**
 * Add an audio file to the queue
 */
export async function addToQueue(audioPath: string): Promise<QueueItem> {
	return tauriInvoke<QueueItem>('add_to_queue_command', { audioPath });
}

/**
 * Remove an item from the queue
 */
export async function removeFromQueue(id: string): Promise<void> {
	return tauriInvoke<void>('remove_from_queue_command', { id });
}

/**
 * Reorder queue items
 */
export async function reorderQueue(fromIndex: number, toIndex: number): Promise<void> {
	return tauriInvoke<void>('reorder_queue_command', { fromIndex, toIndex });
}

/**
 * Clear completed items from the queue
 */
export async function clearCompletedQueue(): Promise<void> {
	return tauriInvoke<void>('clear_completed_queue_command');
}

/**
 * Update a queue item's status
 */
export async function updateQueueItemStatus(id: string, status: QueueItemStatusUpdate): Promise<void> {
	return tauriInvoke<void>('update_queue_item_status_command', { id, status });
}
