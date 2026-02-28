/**
 * Tauri API service layer - type-safe wrappers for Tauri commands
 *
 * This module provides organized access to all Tauri backend commands
 * with proper TypeScript types and error handling.
 */

import { invoke } from '@tauri-apps/api/core';

// Re-export all types
export * from './types';

// Import types for internal use
import type {
	ValidationResult,
	AppHealthResponse,
	AdvancedOptions,
	Preset,
	QueueItem,
	QueueItemStatusUpdate,
	HistoryItem,
	AppSettings,
	PttRouting,
	ShortcutSettings,
	AudioDevice,
	CaptureResult,
	TranscriptionResult
} from './types';

// ============================================================================
// Core API utilities
// ============================================================================

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
 * Error handler wrapper - catches and transforms Tauri errors
 */
export async function safeInvoke<T>(
	command: string,
	args?: Record<string, unknown>
): Promise<{ success: true; data: T } | { success: false; error: string }> {
	try {
		const data = await tauriInvoke<T>(command, args);
		return { success: true, data };
	} catch (e) {
		const error = e as { message?: string; code?: string };
		return { success: false, error: error.message || error.code || 'Unknown error' };
	}
}

// ============================================================================
// File Validation
// ============================================================================

export async function validateModelPath(path: string): Promise<ValidationResult> {
	return tauriInvoke<ValidationResult>('validate_model_path_command', { path });
}

export async function validateAudioPath(path: string): Promise<ValidationResult> {
	return tauriInvoke<ValidationResult>('validate_audio_path_command', { path });
}

// ============================================================================
// File Pickers
// ============================================================================

export async function pickModelPath(): Promise<string> {
	return tauriInvoke<string>('pick_model_path_command');
}

export async function pickAudioPath(): Promise<string> {
	return tauriInvoke<string>('pick_audio_path_command');
}

export async function pickDirectory(): Promise<string> {
	return tauriInvoke<string>('pick_directory_command');
}

// ============================================================================
// App Health
// ============================================================================

export async function getAppHealth(): Promise<AppHealthResponse> {
	return tauriInvoke<AppHealthResponse>('app_health_command');
}

// ============================================================================
// Transcription
// ============================================================================

export async function runTranscription(
	modelPath: string,
	audioPath: string,
	options?: AdvancedOptions
): Promise<TranscriptionResult> {
	return tauriInvoke<TranscriptionResult>('run_transcription_command', {
		modelPath,
		audioPath,
		...options
	});
}

export async function exportTranscript(transcript: string, format: string): Promise<string> {
	return tauriInvoke<string>('export_transcript_command', { transcript, format });
}

export async function openOutputFolder(filePath: string): Promise<void> {
	return tauriInvoke<void>('open_output_folder_command', { filePath });
}

export async function copyToClipboard(text: string): Promise<void> {
	return tauriInvoke<void>('copy_to_clipboard_command', { text });
}

// ============================================================================
// Presets
// ============================================================================

export async function listPresets(): Promise<Preset[]> {
	return tauriInvoke<Preset[]>('list_presets_command');
}

export async function getPreset(name: string): Promise<Preset | null> {
	return tauriInvoke<Preset | null>('get_preset_command', { name });
}

export async function savePreset(name: string, advanced: AdvancedOptions): Promise<void> {
	return tauriInvoke<void>('save_preset_command', { name, advanced });
}

export async function deletePreset(name: string): Promise<void> {
	return tauriInvoke<void>('delete_preset_command', { name });
}

export async function setDefaultPreset(name: string | null): Promise<void> {
	return tauriInvoke<void>('set_default_preset_command', { name });
}

export async function getDefaultPreset(): Promise<string | null> {
	return tauriInvoke<string | null>('get_default_preset_command');
}

// ============================================================================
// Batch Queue
// ============================================================================

export async function getQueue(): Promise<QueueItem[]> {
	return tauriInvoke<QueueItem[]>('get_queue_command');
}

export async function addToQueue(audioPath: string): Promise<QueueItem> {
	return tauriInvoke<QueueItem>('add_to_queue_command', { audioPath });
}

export async function removeFromQueue(id: string): Promise<void> {
	return tauriInvoke<void>('remove_from_queue_command', { id });
}

export async function reorderQueue(fromIndex: number, toIndex: number): Promise<void> {
	return tauriInvoke<void>('reorder_queue_command', { fromIndex, toIndex });
}

export async function clearCompletedQueue(): Promise<void> {
	return tauriInvoke<void>('clear_completed_queue_command');
}

export async function updateQueueItemStatus(id: string, status: QueueItemStatusUpdate): Promise<void> {
	return tauriInvoke<void>('update_queue_item_status_command', { id, status });
}

// ============================================================================
// History
// ============================================================================

export async function getHistory(): Promise<HistoryItem[]> {
	return tauriInvoke<HistoryItem[]>('get_history_command');
}

export async function addToHistory(
	audioPath: string,
	outputPath: string | null,
	success: boolean
): Promise<void> {
	return tauriInvoke<void>('add_to_history_command', { audioPath, outputPath, success });
}

export async function clearHistory(): Promise<void> {
	return tauriInvoke<void>('clear_history_command');
}

// ============================================================================
// App Settings
// ============================================================================

export async function getAppSettings(): Promise<AppSettings> {
	return tauriInvoke<AppSettings>('get_app_settings_command');
}

export async function setTheme(theme: string): Promise<void> {
	return tauriInvoke<void>('set_theme_command', { request: { theme } });
}

export async function setDefaultOutputDir(path: string): Promise<void> {
	return tauriInvoke<void>('set_default_output_dir_command', { request: { path } });
}

export async function setDefaultModelDir(path: string): Promise<void> {
	return tauriInvoke<void>('set_default_model_dir_command', { request: { path } });
}

export async function setDefaultThreads(value: number | null): Promise<void> {
	return tauriInvoke<void>('set_default_threads_command', { request: { value } });
}

export async function setDefaultTimeout(value: number | null): Promise<void> {
	return tauriInvoke<void>('set_default_timeout_command', { request: { value } });
}

export async function setDiagnosticsEnabled(enabled: boolean): Promise<void> {
	return tauriInvoke<void>('set_diagnostics_enabled_command', { request: { enabled } });
}

export async function setStartInBackground(enabled: boolean): Promise<void> {
	return tauriInvoke<void>('set_start_in_background_command', { enabled });
}

export async function setLaunchOnLogin(enabled: boolean): Promise<void> {
	return tauriInvoke<void>('set_launch_on_login_command', { enabled });
}

// ============================================================================
// PTT Routing
// ============================================================================

export async function getPttRouting(): Promise<PttRouting> {
	return tauriInvoke<PttRouting>('get_ptt_routing_command');
}

export async function setPttRouting(routing: PttRouting): Promise<void> {
	return tauriInvoke<void>('set_ptt_routing_command', { routing });
}

// ============================================================================
// Global Shortcuts
// ============================================================================

/**
 * Get shortcut settings from localStorage
 */
export function getShortcuts(): ShortcutSettings {
	try {
		const stored = localStorage.getItem('frontend-tauri.shortcuts');
		const enabled = localStorage.getItem('frontend-tauri.shortcutsEnabled') === 'true';
		if (stored) {
			return { ...JSON.parse(stored), enabled };
		}
	} catch {
		// Ignore
	}
	return {
		enabled: false,
		ptt: 'Ctrl+Shift+Space',
		type: 'Ctrl+Shift+T',
		clipboard: 'Ctrl+Shift+C',
		file: 'Ctrl+Shift+F'
	};
}

/**
 * Save shortcut settings to localStorage
 */
export function setShortcuts(settings: ShortcutSettings): void {
	localStorage.setItem(
		'frontend-tauri.shortcuts',
		JSON.stringify({
			ptt: settings.ptt,
			type: settings.type,
			clipboard: settings.clipboard,
			file: settings.file
		})
	);
	localStorage.setItem('frontend-tauri.shortcutsEnabled', settings.enabled ? 'true' : 'false');
}

export async function registerGlobalShortcut(shortcut: string): Promise<void> {
	return tauriInvoke<void>('register_global_shortcut_command', { shortcut });
}

export async function unregisterGlobalShortcut(shortcut: string): Promise<void> {
	return tauriInvoke<void>('unregister_global_shortcut_command', { shortcut });
}

// ============================================================================
// Audio Capture
// ============================================================================

export async function listAudioDevices(): Promise<AudioDevice[]> {
	return tauriInvoke<AudioDevice[]>('list_audio_devices_command');
}

export async function selectAudioDevice(deviceName: string): Promise<void> {
	return tauriInvoke<void>('select_audio_device_command', { deviceName });
}

export async function startCapture(): Promise<void> {
	return tauriInvoke<void>('start_capture_command');
}

export async function stopCapture(): Promise<CaptureResult> {
	return tauriInvoke<CaptureResult>('stop_capture_command');
}
