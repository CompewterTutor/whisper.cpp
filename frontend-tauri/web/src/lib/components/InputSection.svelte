<script lang="ts">
	import { onMount } from 'svelte';
	import FilePicker from './FilePicker.svelte';
	import {
		pickModelPath,
		pickAudioPath,
		validateModelPath,
		validateAudioPath,
		type ApiError
	} from '$lib/services/tauri';

	// State
	let modelPath = $state('');
	let audioPath = $state('');
	let modelHint = $state('Model not validated yet.');
	let audioHint = $state('Audio not validated yet.');
	let modelHintType = $state<'neutral' | 'ok' | 'error'>('neutral');
	let audioHintType = $state<'neutral' | 'ok' | 'error'>('neutral');
	let modelValid = $state(false);
	let audioValid = $state(false);

	// Storage keys
	const STORAGE_KEYS = {
		modelPath: 'careless.modelPath',
		audioPath: 'careless.audioPath'
	};

	// Expose validation state to parent
	export function getModelState() {
		return { modelPath, audioPath, modelValid, audioValid };
	}

	// Persist state to localStorage
	function persistState() {
		localStorage.setItem(STORAGE_KEYS.modelPath, modelPath);
		localStorage.setItem(STORAGE_KEYS.audioPath, audioPath);
	}

	// Restore state from localStorage
	function restoreState() {
		const storedModelPath = localStorage.getItem(STORAGE_KEYS.modelPath) || '';
		const storedAudioPath = localStorage.getItem(STORAGE_KEYS.audioPath) || '';
		if (storedModelPath) modelPath = storedModelPath;
		if (storedAudioPath) audioPath = storedAudioPath;
	}

	// Reset validation when path changes
	function resetModelValidation() {
		modelValid = false;
		modelHint = 'Model not validated yet.';
		modelHintType = 'neutral';
		persistState();
	}

	function resetAudioValidation() {
		audioValid = false;
		audioHint = 'Audio not validated yet.';
		audioHintType = 'neutral';
		persistState();
	}

	// File picker handlers
	async function handleBrowseModel() {
		try {
			const path = await pickModelPath();
			modelPath = path;
			resetModelValidation();
			// Auto-validate after picking
			await handleValidateModel();
		} catch (error) {
			// User cancelled or error
			console.log('Model pick cancelled or failed:', error);
		}
	}

	async function handleBrowseAudio() {
		try {
			const path = await pickAudioPath();
			audioPath = path;
			resetAudioValidation();
			// Auto-validate after picking
			await handleValidateAudio();
		} catch (error) {
			// User cancelled or error
			console.log('Audio pick cancelled or failed:', error);
		}
	}

	// Validation handlers
	async function handleValidateModel() {
		if (!modelPath.trim()) {
			modelHint = 'Please enter a model path.';
			modelHintType = 'error';
			modelValid = false;
			return;
		}

		try {
			const result = await validateModelPath(modelPath);
			modelValid = true;
			modelHint = `Valid model: ${result.normalized_path}`;
			modelHintType = 'ok';
			persistState();
		} catch (error) {
			modelValid = false;
			const apiError = error as ApiError;
			modelHint = apiError.message || 'Model path is invalid.';
			modelHintType = 'error';
		}
	}

	async function handleValidateAudio() {
		if (!audioPath.trim()) {
			audioHint = 'Please enter an audio path.';
			audioHintType = 'error';
			audioValid = false;
			return;
		}

		try {
			const result = await validateAudioPath(audioPath);
			audioValid = true;
			audioHint = `Valid audio: ${result.normalized_path}`;
			audioHintType = 'ok';
			persistState();
		} catch (error) {
			audioValid = false;
			const apiError = error as ApiError;
			audioHint = apiError.message || 'Audio path is invalid.';
			audioHintType = 'error';
		}
	}

	// Restore state on mount
	onMount(() => {
		restoreState();
		// Auto-validate if paths were restored
		if (modelPath) handleValidateModel();
		if (audioPath) handleValidateAudio();
	});
</script>

<div class="section">
	<div class="section-header">Input Files</div>
	<div class="card">
		<div class="field-group">
			<FilePicker
				id="modelPath"
				placeholder="C:\models\ggml-base.en.bin"
				bind:value={modelPath}
				bind:hint={modelHint}
				bind:hintType={modelHintType}
				bind:valid={modelValid}
				onchange={resetModelValidation}
				onbrowse={handleBrowseModel}
				onvalidate={handleValidateModel}
			>
				{#snippet label()}
					Model path (<code>.bin</code>)
				{/snippet}
			</FilePicker>

			<FilePicker
				id="audioPath"
				placeholder="C:\samples\jfk.wav"
				bind:value={audioPath}
				bind:hint={audioHint}
				bind:hintType={audioHintType}
				bind:valid={audioValid}
				onchange={resetAudioValidation}
				onbrowse={handleBrowseAudio}
				onvalidate={handleValidateAudio}
			>
				{#snippet label()}
					Audio path (<code>.wav</code>, <code>.mp3</code>, <code>.flac</code>, <code>.ogg</code>, <code>.m4a</code>)
				{/snippet}
			</FilePicker>
		</div>
	</div>
</div>

<style>
	code {
		background: var(--bg-tertiary);
		padding: 0.1rem 0.3rem;
		border-radius: var(--radius-sm);
		font-size: 0.85em;
	}
</style>
