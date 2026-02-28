<script lang="ts">
	import { onMount } from 'svelte';
	import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app';
	import { InputSection, OptionsDrawer, TranscriptSection, BatchQueue, HistoryPanel, SettingsPanel, PttInterface } from '$lib/components';
	import type { AdvancedOptions } from '$lib/services/tauri';
	import '$lib/types/tauri.d.ts';

	let appName = $state('loading...');
	let appVersion = $state('loading...');
	let tauriVersion = $state('loading...');
	let currentOptions = $state<AdvancedOptions>({});
	let transcript = $state('');
	let transcriptLoading = $state(false);
	let statusMessage = $state('');
	let statusType = $state<'ok' | 'error' | 'neutral'>('neutral');
	let modelValid = $state(false); // Would come from InputSection in real app
	let modelPath = $state(''); // Would come from InputSection in real app
	let pttMode = $state<'hold' | 'toggle'>('hold'); // Would come from SettingsPanel

	onMount(async () => {
		try {
			appName = await getName();
			appVersion = await getVersion();
			tauriVersion = await getTauriVersion();
		} catch (e) {
			console.error('Tauri API not available:', e);
			appName = 'Not in Tauri';
			appVersion = '-';
			tauriVersion = '-';
		}
	});

	function handleOptionsChange(options: AdvancedOptions) {
		currentOptions = options;
		console.log('Advanced options changed:', options);
	}

	function handleStatus(message: string, type: 'ok' | 'error' | 'neutral') {
		statusMessage = message;
		statusType = type;
	}

	// Test function to simulate transcript
	function loadTestTranscript() {
		transcript = `[00:00:00.000 --> 00:00:03.000]   And so my fellow Americans, ask not what your country can do for you,
[00:00:03.000 --> 00:00:06.000]   ask what you can do for your country.`;
	}

	// Mock run item handler for testing batch queue
	async function handleRunItem(audioPath: string): Promise<string> {
		// Simulate processing delay
		await new Promise((resolve) => setTimeout(resolve, 1000));
		// Return mock transcript
		return `[Mock transcript for ${audioPath}]`;
	}

	// Toggle model valid for testing
	function toggleModelValid() {
		modelValid = !modelValid;
	}

	// Handle PTT transcript
	function handlePttTranscript(pttTranscript: string) {
		if (pttTranscript) {
			transcript = pttTranscript;
			handleStatus('PTT transcript received', 'ok');
		}
	}
</script>

<svelte:head>
	<title>Careless</title>
</svelte:head>

<!-- Input Files Section -->
<InputSection />

<!-- Controls Section with Options Drawer -->
<div class="section">
	<div class="section-header">Controls</div>
	<OptionsDrawer onchange={handleOptionsChange} />
</div>

<!-- Push-to-Talk Interface -->
<PttInterface
	{modelPath}
	{pttMode}
	onstatus={handleStatus}
	ontranscript={handlePttTranscript}
/>

<!-- Transcript Section -->
<TranscriptSection
	bind:transcript
	bind:loading={transcriptLoading}
	onstatus={handleStatus}
/>

<!-- Status Message -->
{#if statusMessage}
	<div class="section">
		<div class="card">
			<p class="status {statusType}">{statusMessage}</p>
		</div>
	</div>
{/if}

<!-- Batch Queue Section -->
<BatchQueue
	{modelValid}
	onstatus={handleStatus}
	onrunitem={handleRunItem}
/>

<!-- History Panel Section -->
<HistoryPanel
	onstatus={handleStatus}
/>

<!-- Settings Panel Section -->
<SettingsPanel
	onstatus={handleStatus}
/>

<!-- App Info Section (for testing) -->
<div class="section">
	<div class="section-header">App Info</div>
	<div class="card">
		<div class="meta">
			<div class="meta-item">
				<span class="meta-label">App Name</span>
				<span class="meta-value">{appName}</span>
			</div>
			<div class="meta-item">
				<span class="meta-label">App Version</span>
				<span class="meta-value">{appVersion}</span>
			</div>
			<div class="meta-item">
				<span class="meta-label">Tauri Version</span>
				<span class="meta-value">{tauriVersion}</span>
			</div>
			<div class="meta-item">
				<span class="meta-label">Framework</span>
				<span class="meta-value">SvelteKit 5</span>
			</div>
		</div>
	</div>
</div>

<!-- UI Components Preview -->
<div class="section">
	<div class="section-header">UI Components Preview</div>
	<div class="card">
		<div class="actions">
			<button type="button">Primary Button</button>
			<button type="button" class="secondary">Secondary Button</button>
			<button type="button" disabled>Disabled Button</button>
			<button type="button" class="secondary" onclick={loadTestTranscript}>Load Test Transcript</button>
			<button
				type="button"
				class="secondary"
				onclick={toggleModelValid}
			>
				{modelValid ? 'Invalidate Model' : 'Validate Model'}
			</button>
		</div>

		<p class="status ok">Success status message</p>
		<p class="status error">Error status message</p>
	</div>
</div>

<style>
	.meta {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: var(--space-2);
		margin-bottom: var(--space-3);
	}

	.meta-item {
		border: 1px solid var(--border-primary);
		background: var(--bg-primary);
		border-radius: var(--radius-md);
		padding: var(--space-2) 0.6rem;
	}

	.meta-label {
		display: block;
		font-size: var(--text-xs);
		color: var(--text-muted);
	}

	.meta-value {
		font-weight: 600;
	}
</style>
