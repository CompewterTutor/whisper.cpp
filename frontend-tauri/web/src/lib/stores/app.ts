/**
 * Application state stores - shared reactive state for components
 */

import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';
import type { AdvancedOptions, PttRouting, ShortcutSettings } from '$lib/services/types';

// ============================================================================
// Input State
// ============================================================================

export const modelPath = writable<string>('');
export const audioPath = writable<string>('');
export const modelValid = writable<boolean>(false);
export const audioValid = writable<boolean>(false);

// Derived: can run transcription
export const canRunTranscription = derived(
	[modelValid, audioValid],
	([$modelValid, $audioValid]) => $modelValid && $audioValid
);

// ============================================================================
// Transcript State
// ============================================================================

export const transcript = writable<string>('');
export const transcriptLoading = writable<boolean>(false);
export const lastExportPath = writable<string>('');

// ============================================================================
// Options State
// ============================================================================

export const advancedOptions = writable<AdvancedOptions>({});

// ============================================================================
// PTT State
// ============================================================================

export type PttMode = 'hold' | 'toggle';

export const pttMode = writable<PttMode>('hold');
export const pttRouting = writable<PttRouting>({
	copy_to_clipboard: true,
	save_to_file: false,
	type_emulation: false
});

// ============================================================================
// Shortcuts State
// ============================================================================

export const shortcuts = writable<ShortcutSettings>({
	enabled: false,
	ptt: 'Ctrl+Shift+Space',
	type: 'Ctrl+Shift+T',
	clipboard: 'Ctrl+Shift+C',
	file: 'Ctrl+Shift+F'
});

// ============================================================================
// UI State
// ============================================================================

export const statusMessage = writable<string>('');
export const statusType = writable<'ok' | 'error' | 'neutral'>('neutral');

// ============================================================================
// Persistence helpers
// ============================================================================

const STORAGE_KEYS = {
	modelPath: 'careless.modelPath',
	audioPath: 'careless.audioPath',
	exportPath: 'careless.lastExportPath',
	advancedOptions: 'careless.advancedOptions',
	pttMode: 'careless.pttMode',
	shortcuts: 'careless.shortcuts',
	shortcutsEnabled: 'careless.shortcutsEnabled'
};

/**
 * Persist a store value to localStorage
 */
export function persistStore<T>(
	store: { subscribe: (run: (value: T) => void) => () => void; set: (value: T) => void },
	key: string
) {
	if (browser) {
		// Load from localStorage on init
		const stored = localStorage.getItem(key);
		if (stored) {
			try {
				store.set(JSON.parse(stored));
			} catch {
				// Ignore parse errors
			}
		}

		// Subscribe to changes and persist
		store.subscribe((value) => {
			if (value !== undefined && value !== null) {
				localStorage.setItem(key, JSON.stringify(value));
			}
		});
	}
}

/**
 * Initialize persistence for all stores
 */
export function initPersistence() {
	if (!browser) return;

	// Load persisted values
	const persistedModelPath = localStorage.getItem(STORAGE_KEYS.modelPath);
	const persistedAudioPath = localStorage.getItem(STORAGE_KEYS.audioPath);
	const persistedExportPath = localStorage.getItem(STORAGE_KEYS.exportPath);
	const persistedOptions = localStorage.getItem(STORAGE_KEYS.advancedOptions);
	const persistedPttMode = localStorage.getItem(STORAGE_KEYS.pttMode);
	const persistedShortcuts = localStorage.getItem(STORAGE_KEYS.shortcuts);
	const persistedShortcutsEnabled = localStorage.getItem(STORAGE_KEYS.shortcutsEnabled);

	if (persistedModelPath) modelPath.set(persistedModelPath);
	if (persistedAudioPath) audioPath.set(persistedAudioPath);
	if (persistedExportPath) lastExportPath.set(persistedExportPath);
	if (persistedOptions) {
		try {
			advancedOptions.set(JSON.parse(persistedOptions));
		} catch { /* ignore */ }
	}
	if (persistedPttMode) pttMode.set(persistedPttMode as PttMode);
	if (persistedShortcuts) {
		try {
			const parsed = JSON.parse(persistedShortcuts);
			shortcuts.update(s => ({ ...s, ...parsed }));
		} catch { /* ignore */ }
	}
	if (persistedShortcutsEnabled) {
		shortcuts.update(s => ({ ...s, enabled: persistedShortcutsEnabled === 'true' }));
	}

	// Subscribe to changes
	modelPath.subscribe(v => localStorage.setItem(STORAGE_KEYS.modelPath, v));
	audioPath.subscribe(v => localStorage.setItem(STORAGE_KEYS.audioPath, v));
	lastExportPath.subscribe(v => localStorage.setItem(STORAGE_KEYS.exportPath, v));
	advancedOptions.subscribe(v => localStorage.setItem(STORAGE_KEYS.advancedOptions, JSON.stringify(v)));
	pttMode.subscribe(v => localStorage.setItem(STORAGE_KEYS.pttMode, v));
	shortcuts.subscribe(v => {
		localStorage.setItem(STORAGE_KEYS.shortcuts, JSON.stringify({
			ptt: v.ptt,
			type: v.type,
			clipboard: v.clipboard,
			file: v.file
		}));
		localStorage.setItem(STORAGE_KEYS.shortcutsEnabled, v.enabled ? 'true' : 'false');
	});
}

/**
 * Clear all persisted state
 */
export function clearPersistedState() {
	if (!browser) return;
	Object.values(STORAGE_KEYS).forEach(key => localStorage.removeItem(key));
}
