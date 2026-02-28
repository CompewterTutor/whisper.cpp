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
	import {
		modelPath,
		audioPath,
		modelValid,
		audioValid
	} from '$lib/stores';

	// Local UI state for hints
	let modelHint = $state('Model not validated yet.');
	let audioHint = $state('Audio not validated yet.');
	let modelHintType = $state<'neutral' | 'ok' | 'error'>('neutral');
	let audioHintType = $state<'neutral' | 'ok' | 'error'>('neutral');

	// Reset validation when path changes
	function resetModelValidation() {
		modelValid.set(false);
		modelHint = 'Model not validated yet.';
		modelHintType = 'neutral';
	}

	function resetAudioValidation() {
		audioValid.set(false);
		audioHint = 'Audio not validated yet.';
		audioHintType = 'neutral';
	}

	// File picker handlers
	async function handleBrowseModel() {
		try {
			const path = await pickModelPath();
			modelPath.set(path);
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
			audioPath.set(path);
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
		const currentPath = $modelPath;
		if (!currentPath.trim()) {
			modelHint = 'Please enter a model path.';
			modelHintType = 'error';
			modelValid.set(false);
			return;
		}

		try {
			const result = await validateModelPath(currentPath);
			modelValid.set(true);
			modelHint = `Valid model: ${result.normalized_path}`;
			modelHintType = 'ok';
		} catch (error) {
			modelValid.set(false);
			const apiError = error as ApiError;
			modelHint = apiError.message || 'Model path is invalid.';
			modelHintType = 'error';
		}
	}

	async function handleValidateAudio() {
		const currentPath = $audioPath;
		if (!currentPath.trim()) {
			audioHint = 'Please enter an audio path.';
			audioHintType = 'error';
			audioValid.set(false);
			return;
		}

		try {
			const result = await validateAudioPath(currentPath);
			audioValid.set(true);
			audioHint = `Valid audio: ${result.normalized_path}`;
			audioHintType = 'ok';
		} catch (error) {
			audioValid.set(false);
			const apiError = error as ApiError;
			audioHint = apiError.message || 'Audio path is invalid.';
			audioHintType = 'error';
		}
	}

	// Update path from input
	function handleModelChange(value: string) {
		modelPath.set(value);
		resetModelValidation();
	}

	function handleAudioChange(value: string) {
		audioPath.set(value);
		resetAudioValidation();
	}

	// Auto-validate on mount if paths exist
	onMount(() => {
		if ($modelPath) handleValidateModel();
		if ($audioPath) handleValidateAudio();
	});
</script>

<div class="section">
	<div class="section-header">Input Files</div>
	<div class="card">
		<div class="field-group">
			<FilePicker
				id="modelPath"
				placeholder="C:\models\ggml-base.en.bin"
				value={$modelPath}
				{modelHint}
				{modelHintType}
				valid={$modelValid}
				onchange={handleModelChange}
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
				value={$audioPath}
				{audioHint}
				{audioHintType}
				valid={$audioValid}
				onchange={handleAudioChange}
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
