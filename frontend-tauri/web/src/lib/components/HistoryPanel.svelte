<script lang="ts">
	import { onMount } from 'svelte';
	import {
		getHistory,
		clearHistory,
		type HistoryItem,
		type ApiError
	} from '$lib/services/tauri';

	// Props
	interface Props {
		/** Called when status message should be shown */
		onstatus?: (message: string, type: 'ok' | 'error' | 'neutral') => void;
		/** Called when rerun is requested with an audio path */
		onrerun?: (audioPath: string) => void;
	}

	let { onstatus, onrerun }: Props = $props();

	// State
	let history = $state<HistoryItem[]>([]);

	// Computed
	let hasHistory = $derived(history.length > 0);

	// Load history from backend
	async function loadHistory() {
		try {
			history = await getHistory();
		} catch (error) {
			console.warn('Failed to load history:', error);
		}
	}

	// Extract filename from path
	function getFileName(path: string): string {
		return path.split(/[/\\]/).pop() || path;
	}

	// Format timestamp to time string
	function formatTime(timestampMs: number): string {
		const date = new Date(timestampMs);
		return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
	}

	// Handle clear history
	async function handleClearHistory() {
		if (!confirm('Clear all history?')) return;

		try {
			await clearHistory();
			history = [];
			onstatus?.('History cleared', 'ok');
		} catch (error) {
			const apiError = error as ApiError;
			onstatus?.(`Failed to clear history: ${apiError.message}`, 'error');
		}
	}

	// Handle rerun
	function handleRerun(item: HistoryItem) {
		onrerun?.(item.audio_path);
		onstatus?.(`Loaded ${getFileName(item.audio_path)} from history`, 'ok');
	}

	// Initialize on mount
	onMount(() => {
		loadHistory();
	});
</script>

<div class="section">
	<div class="section-header">History</div>
	<div class="card">
		<div class="actions">
			<button
				type="button"
				class="secondary"
				disabled={!hasHistory}
				onclick={handleClearHistory}
			>
				Clear history
			</button>
		</div>

		<div class="history-list">
			{#if !hasHistory}
				<p class="empty-list">No transcription history yet.</p>
			{:else}
				{#each history as item (item.id)}
					<div class="history-item">
						<span class="history-item-time">{formatTime(item.timestamp_ms)}</span>
						<span class="history-item-path" title={item.audio_path}>
							{getFileName(item.audio_path)}
						</span>
						<span class="history-item-status success" class:error={!item.success}>
							{item.success ? '✓' : '✗'}
						</span>
						<div class="history-item-actions">
							<button type="button" class="secondary small" onclick={() => handleRerun(item)}>
								Rerun
							</button>
						</div>
					</div>
				{/each}
			{/if}
		</div>
	</div>
</div>

<style>
	.history-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		max-height: 200px;
		overflow-y: auto;
	}

	.history-item {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) 0.6rem;
		background: var(--bg-primary);
		border: 1px solid var(--border-primary);
		border-radius: var(--radius-sm);
		font-size: 0.85rem;
	}

	.history-item-time {
		font-size: 0.75rem;
		color: var(--text-muted);
		min-width: 80px;
	}

	.history-item-path {
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.history-item-status {
		font-size: 0.75rem;
		padding: 0.2rem 0.4rem;
		border-radius: var(--radius-sm);
		background: #166534;
		color: white;
	}

	.history-item-status.error {
		background: #991b1b;
	}

	.history-item-actions {
		display: flex;
		gap: var(--space-1);
	}

	.history-item-actions button.small {
		padding: 0.3rem 0.5rem;
		font-size: 0.75rem;
	}

	.empty-list {
		text-align: center;
		color: var(--text-muted);
		padding: var(--space-4);
		font-size: 0.9rem;
	}
</style>
