<script lang="ts">
	import { onMount } from 'svelte';
	import {
		listPresets,
		savePreset,
		deletePreset,
		setDefaultPreset,
		getDefaultPreset,
		type AdvancedOptions,
		type Preset
	} from '$lib/services/tauri';

	// Props
	interface Props {
		/** Initial open state */
		open?: boolean;
		/** Called when options change */
		onchange?: (options: AdvancedOptions) => void;
	}

	let { open = $bindable(false), onchange }: Props = $props();

	// State
	let presets = $state<Preset[]>([]);
	let selectedPreset = $state('');
	let isDefaultPreset = $state(false);

	// Advanced options state
	let task = $state('transcribe');
	let language = $state('auto');
	let threads = $state('');
	let beamSize = $state('');
	let bestOf = $state('');
	let temperature = $state('');

	// Storage key
	const STORAGE_KEY = 'careless.advancedOptions';

	// Get current options as object
	function getOptions(): AdvancedOptions {
		const threadsNum = threads ? parseInt(threads, 10) : null;
		const beamSizeNum = beamSize ? parseInt(beamSize, 10) : null;
		const bestOfNum = bestOf ? parseInt(bestOf, 10) : null;
		const tempFloat = temperature ? parseFloat(temperature) : null;

		return {
			task: task || null,
			language: language || null,
			threads: threadsNum && threadsNum > 0 ? threadsNum : null,
			beam_size: beamSizeNum && beamSizeNum > 0 ? beamSizeNum : null,
			best_of: bestOfNum && bestOfNum > 0 ? bestOfNum : null,
			temperature: tempFloat !== null ? Math.round(tempFloat * 100) : null
		};
	}

	// Set options from object
	function setOptions(options: AdvancedOptions) {
		task = options.task || 'transcribe';
		language = options.language || 'auto';
		threads = options.threads?.toString() || '';
		beamSize = options.beam_size?.toString() || '';
		bestOf = options.best_of?.toString() || '';
		temperature = options.temperature !== null && options.temperature !== undefined
			? (options.temperature / 100).toFixed(2)
			: '';
	}

	// Persist options to localStorage
	function persistOptions() {
		const options = getOptions();
		localStorage.setItem(STORAGE_KEY, JSON.stringify(options));
		onchange?.(options);
	}

	// Restore options from localStorage
	function restoreOptions() {
		const stored = localStorage.getItem(STORAGE_KEY);
		if (stored) {
			try {
				const options = JSON.parse(stored) as AdvancedOptions;
				setOptions(options);
			} catch (e) {
				console.warn('Failed to restore advanced options:', e);
			}
		}
	}

	// Load presets from backend
	async function loadPresets() {
		try {
			presets = await listPresets();
			updatePresetDropdown();

			// Load default preset
			const defaultPresetName = await getDefaultPreset();
			if (defaultPresetName) {
				selectedPreset = defaultPresetName;
				const preset = presets.find((p) => p.name === defaultPresetName);
				if (preset) {
					setOptions(preset.advanced);
					isDefaultPreset = true;
				}
			}
		} catch (error) {
			console.warn('Failed to load presets:', error);
		}
	}

	// Update preset dropdown options
	function updatePresetDropdown() {
		// The select element will re-render when `presets` changes
	}

	// Handle preset selection
	function handlePresetChange() {
		if (selectedPreset) {
			const preset = presets.find((p) => p.name === selectedPreset);
			if (preset) {
				setOptions(preset.advanced);
				persistOptions();
			}
		}
	}

	// Save current options as a new preset
	async function handleSavePreset() {
		const name = prompt('Enter preset name:');
		if (!name || !name.trim()) return;

		const options = getOptions();
		try {
			await savePreset(name.trim(), options);
			await loadPresets();
			selectedPreset = name.trim();
		} catch (error) {
			console.error('Failed to save preset:', error);
			alert('Failed to save preset: ' + (error as Error).message);
		}
	}

	// Delete selected preset
	async function handleDeletePreset() {
		if (!selectedPreset) return;
		if (!confirm(`Delete preset "${selectedPreset}"?`)) return;

		try {
			await deletePreset(selectedPreset);
			await loadPresets();
			selectedPreset = '';
			isDefaultPreset = false;
		} catch (error) {
			console.error('Failed to delete preset:', error);
			alert('Failed to delete preset: ' + (error as Error).message);
		}
	}

	// Toggle default preset
	async function handleToggleDefault() {
		try {
			await setDefaultPreset(isDefaultPreset ? selectedPreset : null);
		} catch (error) {
			console.error('Failed to set default preset:', error);
			isDefaultPreset = !isDefaultPreset; // Revert
		}
	}

	// Toggle drawer open/closed
	function toggleDrawer() {
		open = !open;
	}

	// Initialize on mount
	onMount(() => {
		restoreOptions();
		loadPresets();
	});
</script>

<div class="card">
	<button
		type="button"
		class="drawer-toggle"
		class:open
		onclick={toggleDrawer}
		aria-expanded={open}
		aria-controls="advancedDrawer"
	>
		Advanced Options
	</button>
	<div id="advancedDrawer" class="drawer-content" class:open>
		<!-- Preset Management -->
		<div class="preset-row">
			<div class="field">
				<label for="presetSelect">Preset</label>
				<select id="presetSelect" bind:value={selectedPreset} onchange={handlePresetChange}>
					<option value="">-- Select preset --</option>
					{#each presets as preset}
						<option value={preset.name}>{preset.name}</option>
					{/each}
				</select>
			</div>
			<button type="button" class="secondary" onclick={handleSavePreset}>Save As...</button>
			<button type="button" class="secondary" disabled={!selectedPreset} onclick={handleDeletePreset}>Delete</button>
			<label class="toggle">
				<input
					type="checkbox"
					bind:checked={isDefaultPreset}
					disabled={!selectedPreset}
					onchange={handleToggleDefault}
				/>
				<span>Default</span>
			</label>
		</div>

		<!-- Options Grid -->
		<div class="options-grid">
			<div class="field">
				<label for="taskSelect">Task</label>
				<select id="taskSelect" bind:value={task} onchange={persistOptions}>
					<option value="transcribe">Transcribe</option>
					<option value="translate">Translate to English</option>
				</select>
			</div>
			<div class="field">
				<label for="languageSelect">Language</label>
				<select id="languageSelect" bind:value={language} onchange={persistOptions}>
					<option value="auto">Auto-detect</option>
					<option value="en">English</option>
					<option value="fr">French</option>
					<option value="de">German</option>
					<option value="es">Spanish</option>
					<option value="it">Italian</option>
					<option value="pt">Portuguese</option>
					<option value="ja">Japanese</option>
					<option value="ko">Korean</option>
					<option value="zh">Chinese</option>
					<option value="ru">Russian</option>
				</select>
			</div>
			<div class="field">
				<label for="threadsInput">Threads</label>
				<input
					id="threadsInput"
					type="number"
					min="1"
					max="64"
					placeholder="Auto"
					bind:value={threads}
					onchange={persistOptions}
				/>
			</div>
			<div class="field">
				<label for="beamSizeInput">Beam size</label>
				<input
					id="beamSizeInput"
					type="number"
					min="1"
					max="10"
					placeholder="5"
					bind:value={beamSize}
					onchange={persistOptions}
				/>
			</div>
			<div class="field">
				<label for="bestOfInput">Best of</label>
				<input
					id="bestOfInput"
					type="number"
					min="1"
					max="10"
					placeholder="5"
					bind:value={bestOf}
					onchange={persistOptions}
				/>
			</div>
			<div class="field">
				<label for="temperatureInput">Temperature</label>
				<input
					id="temperatureInput"
					type="number"
					min="0"
					max="1"
					step="0.1"
					placeholder="0.0"
					bind:value={temperature}
					onchange={persistOptions}
				/>
			</div>
		</div>
	</div>
</div>

<style>
	.drawer-toggle {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		background: transparent;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		font-size: 0.85rem;
		padding: var(--space-2) 0;
		margin-bottom: var(--space-2);
		width: 100%;
		text-align: left;
	}

	.drawer-toggle:hover {
		color: var(--text-primary);
	}

	.drawer-toggle::before {
		content: "▶";
		font-size: 0.7rem;
		transition: transform 0.2s;
	}

	.drawer-toggle.open::before {
		transform: rotate(90deg);
	}

	.drawer-content {
		display: none;
	}

	.drawer-content.open {
		display: block;
	}

	.preset-row {
		display: flex;
		gap: var(--space-2);
		align-items: flex-end;
		margin-bottom: var(--space-3);
		flex-wrap: wrap;
	}

	.preset-row .field {
		flex: 1;
		min-width: 150px;
	}

	.toggle {
		display: flex;
		gap: var(--space-2);
		align-items: center;
		margin-bottom: 0.3rem;
	}

	.toggle span {
		font-size: 0.8rem;
	}

	.toggle input[type="checkbox"] {
		width: auto;
		margin: 0;
	}

	.options-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-3);
	}

	@media (max-width: 600px) {
		.options-grid {
			grid-template-columns: 1fr;
		}
	}

	.options-grid .field {
		gap: var(--space-1);
	}

	.options-grid label {
		font-size: 0.8rem;
	}

	.options-grid input,
	.options-grid select {
		font-size: 0.9rem;
		padding: var(--space-1) var(--space-2);
	}
</style>
