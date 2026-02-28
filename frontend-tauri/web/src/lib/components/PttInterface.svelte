<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		startCapture as apiStartCapture,
		stopCapture as apiStopCapture,
		type PttState
	} from '$lib/services/tauri';

	interface Props {
		modelPath?: string;
		pttMode?: 'hold' | 'toggle';
		onstatus?: (message: string, type: 'ok' | 'error' | 'neutral') => void;
		ontranscript?: (transcript: string) => void;
	}

	let { modelPath = '', pttMode = 'hold', onstatus, ontranscript }: Props = $props();

	// PTT state
	let pttState = $state<PttState>('idle');
	let statusMessage = $state('Ready');
	let timerDisplay = $state('');
	let startTime = $state<number | null>(null);
	let timerInterval = $state<ReturnType<typeof setInterval> | null>(null);

	// Expose state for parent components
	let isRecording = $derived(pttState === 'listening');
	let isTranscribing = $derived(pttState === 'transcribing');
	let isActive = $derived(pttState !== 'idle');

	onMount(() => {
		// Listen for PTT events from backend (global shortcut triggers)
		window.addEventListener('ptt-start', handlePttStart);
		window.addEventListener('ptt-stop', handlePttStop);
	});

	onDestroy(() => {
		window.removeEventListener('ptt-start', handlePttStart);
		window.removeEventListener('ptt-stop', handlePttStop);
		if (timerInterval) {
			clearInterval(timerInterval);
		}
	});

	function updateTimer() {
		if (startTime) {
			const elapsed = Math.floor((Date.now() - startTime) / 1000);
			const minutes = Math.floor(elapsed / 60);
			const seconds = elapsed % 60;
			timerDisplay = `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
		}
	}

	function updateStatus(state: PttState, message?: string) {
		pttState = state;

		switch (state) {
			case 'listening':
				statusMessage = 'Recording...';
				startTime = Date.now();
				timerInterval = setInterval(updateTimer, 100);
				break;
			case 'transcribing':
				statusMessage = 'Transcribing...';
				if (timerInterval) {
					clearInterval(timerInterval);
					timerInterval = null;
				}
				timerDisplay = '';
				break;
			case 'error':
				statusMessage = message || 'Error';
				if (timerInterval) {
					clearInterval(timerInterval);
					timerInterval = null;
				}
				timerDisplay = '';
				// Auto-hide after 3 seconds
				setTimeout(() => {
					if (pttState === 'error') {
						pttState = 'idle';
						statusMessage = 'Ready';
					}
				}, 3000);
				break;
			case 'idle':
			default:
				statusMessage = 'Ready';
				if (timerInterval) {
					clearInterval(timerInterval);
					timerInterval = null;
				}
				timerDisplay = '';
				break;
		}
	}

	async function handlePttStart() {
		if (pttMode === 'hold') {
			await beginCapture();
		} else {
			// Toggle mode - only start if idle
			if (pttState === 'idle') {
				await beginCapture();
			}
		}
	}

	async function handlePttStop() {
		if (pttMode === 'hold') {
			await stopCaptureAndTranscribe();
		}
		// In toggle mode, we don't stop on release - user needs to press again
	}

	async function beginCapture() {
		if (pttState !== 'idle') return;

		try {
			updateStatus('listening');
			await apiStartCapture();
			onstatus?.('Capture started', 'neutral');
		} catch (error: unknown) {
			const err = error as { message?: string };
			updateStatus('error', err.message || 'Failed to start capture');
			onstatus?.('Failed to start capture', 'error');
		}
	}

	async function stopCaptureAndTranscribe() {
		if (pttState !== 'listening') return;

		try {
			updateStatus('transcribing');

			if (!modelPath.trim()) {
				throw new Error('No model selected');
			}

			// Stop capture - the backend should return the transcript
			const result = await apiStopCapture();
			const transcript = result?.transcript || '';

			onstatus?.('Capture stopped. Transcription complete.', 'ok');
			ontranscript?.(transcript);
			updateStatus('idle');
		} catch (error: unknown) {
			const err = error as { message?: string };
			updateStatus('error', err.message || 'Failed to stop capture');
			onstatus?.('Failed to stop capture', 'error');
		}
	}

	async function toggleCapture() {
		if (pttState === 'idle') {
			await beginCapture();
		} else if (pttState === 'listening') {
			await stopCaptureAndTranscribe();
		}
	}

	// Reset to idle state
	function reset() {
		if (timerInterval) {
			clearInterval(timerInterval);
			timerInterval = null;
		}
		pttState = 'idle';
		statusMessage = 'Ready';
		timerDisplay = '';
		startTime = null;
	}
</script>

<div class="section">
	<div class="section-header">Push-to-Talk</div>
	<div class="card">
		<div class="ptt-status">
			<div class="ptt-status-indicator">
				<span class="ptt-dot" class:listening={pttState === 'listening'} class:transcribing={pttState === 'transcribing'} class:error={pttState === 'error'}></span>
				<span class="ptt-status-text">{statusMessage}</span>
				{#if timerDisplay}
					<span class="ptt-timer">{timerDisplay}</span>
				{/if}
			</div>
		</div>

		<div class="ptt-controls">
			{#if pttMode === 'toggle'}
				<button
					type="button"
					onclick={toggleCapture}
					disabled={pttState === 'transcribing'}
				>
					{#if pttState === 'listening'}
						Stop & Transcribe
					{:else}
						Start Recording
					{/if}
				</button>
			{:else}
				<button
					type="button"
					onmousedown={beginCapture}
					onmouseup={stopCaptureAndTranscribe}
					onmouseleave={pttState === 'listening' ? stopCaptureAndTranscribe : undefined}
					disabled={pttState === 'transcribing'}
				>
					{#if pttState === 'listening'}
						Recording... Release to Transcribe
					{:else if pttState === 'transcribing'}
						Transcribing...
					{:else}
						Hold to Record
					{/if}
				</button>
			{/if}

			{#if pttState === 'error'}
				<button type="button" class="secondary" onclick={reset}>
					Reset
				</button>
			{/if}
		</div>

		<p class="hint neutral">
			{pttMode === 'hold'
				? 'Hold the button or use the global shortcut to record.'
				: 'Click the button or use the global shortcut to toggle recording.'}
		</p>
	</div>
</div>

<style>
	.ptt-status {
		padding: var(--space-3);
		background: var(--bg-secondary);
		border-radius: var(--radius-md);
		border: 1px solid var(--border-primary);
		margin-bottom: var(--space-3);
	}

	.ptt-status-indicator {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.ptt-dot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background: var(--text-muted);
		transition: background-color 0.2s;
	}

	.ptt-dot.listening {
		background: #ef4444;
		animation: pulse 1s infinite;
	}

	.ptt-dot.transcribing {
		background: #f59e0b;
		animation: pulse 0.5s infinite;
	}

	.ptt-dot.error {
		background: #ef4444;
	}

	@keyframes pulse {
		0%, 100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.ptt-status-text {
		font-size: var(--text-sm);
		font-weight: 500;
	}

	.ptt-timer {
		margin-left: auto;
		font-family: monospace;
		font-size: var(--text-sm);
		color: var(--text-muted);
	}

	.ptt-controls {
		display: flex;
		gap: var(--space-2);
	}

	.hint {
		min-height: 1.1rem;
		font-size: var(--text-xs);
		margin-top: var(--space-2);
	}

	.hint.neutral {
		color: var(--text-muted);
	}
</style>
