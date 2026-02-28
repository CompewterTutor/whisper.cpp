<script lang="ts">
	import { onMount } from 'svelte';
	import {
		getQueue,
		addToQueue,
		removeFromQueue,
		reorderQueue,
		clearCompletedQueue,
		pickAudioPath,
		type QueueItem,
		type QueueItemStatus,
		type ApiError
	} from '$lib/services/tauri';

	// Props
	interface Props {
		/** Whether a valid model is selected (enables Run All) */
		modelValid?: boolean;
		/** Called when status message should be shown */
		onstatus?: (message: string, type: 'ok' | 'error' | 'neutral') => void;
		/** Called when running queue items */
		onrunitem?: (audioPath: string) => Promise<string>;
	}

	let {
		modelValid = false,
		onstatus,
		onrunitem
	}: Props = $props();

	// State
	let queue = $state<QueueItem[]>([]);
	let isQueueRunning = $state(false);

	// Computed
	let hasPending = $derived(queue.some((item) => item.status === 'Pending'));
	let hasCompleted = $derived(queue.some((item) => item.status === 'Success' || item.status === 'Error'));
	let canRunAll = $derived(hasPending && !isQueueRunning && modelValid);

	// Load queue from backend
	async function loadQueue() {
		try {
			queue = await getQueue();
		} catch (error) {
			console.warn('Failed to load queue:', error);
		}
	}

	// Extract filename from path
	function getFileName(path: string): string {
		return path.split(/[/\\]/).pop() || path;
	}

	// Handle add files
	async function handleAddFiles() {
		try {
			const path = await pickAudioPath();
			const item = await addToQueue(path);
			queue.push(item);
			onstatus?.(`Added ${getFileName(path)} to queue`, 'ok');
		} catch (error) {
			const apiError = error as ApiError;
			if (apiError.code !== 'selection_cancelled') {
				onstatus?.(`Failed to add file: ${apiError.message}`, 'error');
			}
		}
	}

	// Handle remove item
	async function handleRemove(id: string) {
		try {
			await removeFromQueue(id);
			queue = queue.filter((item) => item.id !== id);
		} catch (error) {
			const apiError = error as ApiError;
			onstatus?.(`Failed to remove: ${apiError.message}`, 'error');
		}
	}

	// Handle reorder up
	async function handleMoveUp(index: number) {
		if (index <= 0) return;
		try {
			await reorderQueue(index, index - 1);
			const item = queue.splice(index, 1)[0];
			queue.splice(index - 1, 0, item);
		} catch (error) {
			const apiError = error as ApiError;
			onstatus?.(`Failed to reorder: ${apiError.message}`, 'error');
		}
	}

	// Handle reorder down
	async function handleMoveDown(index: number) {
		if (index >= queue.length - 1) return;
		try {
			await reorderQueue(index, index + 1);
			const item = queue.splice(index, 1)[0];
			queue.splice(index + 1, 0, item);
		} catch (error) {
			const apiError = error as ApiError;
			onstatus?.(`Failed to reorder: ${apiError.message}`, 'error');
		}
	}

	// Handle clear completed
	async function handleClearCompleted() {
		try {
			await clearCompletedQueue();
			queue = queue.filter((item) => item.status !== 'Success' && item.status !== 'Error');
			onstatus?.('Cleared completed items from queue', 'ok');
		} catch (error) {
			const apiError = error as ApiError;
			onstatus?.(`Failed to clear: ${apiError.message}`, 'error');
		}
	}

	// Handle run all
	async function handleRunAll() {
		if (isQueueRunning || !modelValid || !onrunitem) return;

		isQueueRunning = true;
		const pendingItems = queue.filter((item) => item.status === 'Pending');
		let successCount = 0;
		let errorCount = 0;

		for (const item of pendingItems) {
			// Update status to running
			item.status = 'Running' as QueueItemStatus;
			queue = queue; // Trigger reactivity

			try {
				await onrunitem(item.audio_path);
				item.status = 'Success' as QueueItemStatus;
				successCount++;
			} catch (error) {
				item.status = 'Error' as QueueItemStatus;
				item.error_message = (error as Error).message;
				errorCount++;
			}
		}

		isQueueRunning = false;
		onstatus?.(
			`Batch complete: ${successCount} succeeded, ${errorCount} failed`,
			errorCount === 0 ? 'ok' : 'error'
		);
	}

	// Get status class
	function getStatusClass(status: QueueItemStatus): string {
		return status.toLowerCase();
	}

	// Initialize on mount
	onMount(() => {
		loadQueue();
	});
</script>

<div class="section">
	<div class="section-header">Batch Queue</div>
	<div class="card">
		<div class="actions">
			<button type="button" class="secondary" onclick={handleAddFiles}>Add files...</button>
			<button type="button" disabled={!canRunAll} onclick={handleRunAll}>
				{isQueueRunning ? 'Running...' : 'Run all'}
			</button>
			<button
				type="button"
				class="secondary"
				disabled={!hasCompleted || isQueueRunning}
				onclick={handleClearCompleted}
			>
				Clear completed
			</button>
		</div>

		<div class="queue-list">
			{#if queue.length === 0}
				<p class="empty-list">No files in queue. Add audio files to batch process.</p>
			{:else}
				{#each queue as item, index (item.id)}
					<div class="queue-item">
						<span class="queue-item-path" title={item.audio_path}>
							{getFileName(item.audio_path)}
						</span>
						<span class="queue-item-status {getStatusClass(item.status)}">
							{item.status}
						</span>
						<div class="queue-item-actions">
							{#if index > 0}
								<button type="button" class="secondary small" onclick={() => handleMoveUp(index)}>↑</button>
							{/if}
							{#if index < queue.length - 1}
								<button type="button" class="secondary small" onclick={() => handleMoveDown(index)}>↓</button>
							{/if}
							<button type="button" class="secondary small" onclick={() => handleRemove(item.id)}>×</button>
						</div>
					</div>
				{/each}
			{/if}
		</div>
	</div>
</div>

<style>
	.queue-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		max-height: 200px;
		overflow-y: auto;
	}

	.queue-item {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) 0.6rem;
		background: var(--bg-primary);
		border: 1px solid var(--border-primary);
		border-radius: var(--radius-sm);
		font-size: 0.85rem;
	}

	.queue-item-path {
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.queue-item-status {
		font-size: 0.75rem;
		padding: 0.2rem 0.4rem;
		border-radius: var(--radius-sm);
		background: var(--bg-tertiary);
		text-transform: none;
	}

	.queue-item-status.pending {
		background: var(--bg-tertiary);
		color: var(--text-muted);
	}

	.queue-item-status.running {
		background: #1d4ed8;
		color: white;
	}

	.queue-item-status.success {
		background: #166534;
		color: white;
	}

	.queue-item-status.error {
		background: #991b1b;
		color: white;
	}

	.queue-item-actions {
		display: flex;
		gap: var(--space-1);
	}

	.queue-item-actions button.small {
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
