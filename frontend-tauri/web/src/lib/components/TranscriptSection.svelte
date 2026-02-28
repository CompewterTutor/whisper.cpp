<script lang="ts">
	import {
		exportTranscript,
		openOutputFolder,
		copyToClipboard,
		type ApiError
	} from '$lib/services/tauri';

	// Props
	interface Props {
		/** Current transcript text */
		transcript?: string;
		/** Whether transcription is in progress */
		loading?: boolean;
		/** Called when status message should be shown */
		onstatus?: (message: string, type: 'ok' | 'error' | 'neutral') => void;
	}

	let {
		transcript = $bindable(''),
		loading = $bindable(false),
		onstatus
	}: Props = $props();

	// State
	let segmentCount = $state(0);
	let estimatedDuration = $state('0.0s');
	let lastExportPath = $state('');

	// Storage key
	const STORAGE_KEY = 'careless.lastExportPath';

	// Update metadata when transcript changes
	function updateMetadata(text: string) {
		const lines = text
			.split(/\r?\n/)
			.map((line) => line.trim())
			.filter(Boolean);
		segmentCount = lines.length;
		const estimatedSeconds = segmentCount * 2;
		estimatedDuration = `${estimatedSeconds.toFixed(1)}s`;
	}

	// Restore last export path
	function restoreState() {
		const stored = localStorage.getItem(STORAGE_KEY);
		if (stored) {
			lastExportPath = stored;
		}
	}

	// Handle copy to clipboard
	async function handleCopy() {
		const text = transcript.trim();
		if (!text) {
			onstatus?.('No transcript to copy.', 'error');
			return;
		}

		try {
			await copyToClipboard(text);
			onstatus?.('Copied to clipboard.', 'ok');
		} catch (error) {
			const apiError = error as ApiError;
			onstatus?.(`Failed to copy: ${apiError.message}`, 'error');
		}
	}

	// Handle export
	async function handleExport(format: string) {
		const text = transcript.trim();
		if (!text) {
			onstatus?.('No transcript to export.', 'error');
			return;
		}

		try {
			onstatus?.(`Exporting ${format.toUpperCase()}...`, 'neutral');
			const savedPath = await exportTranscript(text, format);
			lastExportPath = savedPath;
			localStorage.setItem(STORAGE_KEY, savedPath);
			onstatus?.(`Exported ${format.toUpperCase()} to ${savedPath}`, 'ok');
		} catch (error) {
			const apiError = error as ApiError;
			onstatus?.(`Export ${format.toUpperCase()} failed: ${apiError.message}`, 'error');
		}
	}

	// Handle open output folder
	async function handleOpenFolder() {
		if (!lastExportPath) {
			onstatus?.('No exported file yet.', 'error');
			return;
		}

		try {
			await openOutputFolder(lastExportPath);
			onstatus?.('Opened output folder.', 'ok');
		} catch (error) {
			const apiError = error as ApiError;
			onstatus?.(`Failed to open folder: ${apiError.message}`, 'error');
		}
	}

	// Watch for transcript changes
	$effect(() => {
		updateMetadata(transcript);
	});

	// Initialize on mount
	$effect(() => {
		restoreState();
	});
</script>

<div class="section">
	<div class="section-header">Transcript</div>
	<div class="card">
		<!-- Metadata -->
		<div class="meta">
			<div class="meta-item">
				<span class="meta-label">Segments</span>
				<span class="meta-value">{segmentCount}</span>
			</div>
			<div class="meta-item">
				<span class="meta-label">Estimated duration</span>
				<span class="meta-value">{estimatedDuration}</span>
			</div>
		</div>

		<!-- Actions -->
		<div class="actions">
			<button type="button" class="secondary" onclick={handleCopy}>Copy</button>
			<button type="button" class="secondary" onclick={() => handleExport('txt')}>Export TXT</button>
			<button type="button" class="secondary" onclick={() => handleExport('srt')}>Export SRT</button>
			<button type="button" class="secondary" onclick={() => handleExport('vtt')}>Export VTT</button>
			<button type="button" class="secondary" onclick={() => handleExport('json')}>Export JSON</button>
			<button
				type="button"
				class="secondary"
				disabled={!lastExportPath}
				onclick={handleOpenFolder}
			>
				Open output folder
			</button>
		</div>

		<!-- Transcript Viewer -->
		<pre class="transcript" class:loading>
			{#if loading}
				(running...)
			{:else if transcript.trim()}
				{transcript}
			{:else}
				(no transcript yet)
			{/if}
		</pre>
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

	.actions {
		display: flex;
		gap: var(--space-2);
		flex-wrap: wrap;
		margin-bottom: var(--space-3);
	}

	.transcript {
		white-space: pre-wrap;
		margin: 0;
		min-height: 7rem;
		border-radius: var(--radius-md);
		background: var(--bg-primary);
		border: 1px solid var(--border-primary);
		padding: var(--space-3);
		color: var(--text-secondary);
		font-family: inherit;
		font-size: 0.9rem;
		overflow: auto;
		max-height: 400px;
	}

	.transcript.loading {
		opacity: 0.7;
		font-style: italic;
	}
</style>
