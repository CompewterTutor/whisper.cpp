<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		/** Input element id */
		id: string;
		/** Label text */
		label: Snippet;
		/** Placeholder text for input */
		placeholder?: string;
		/** Current file path value */
		value?: string;
		/** Validation hint message */
		hint?: string;
		/** Hint type: neutral, ok, or error */
		hintType?: 'neutral' | 'ok' | 'error';
		/** Whether the path is validated */
		valid?: boolean;
		/** Called when path changes */
		onchange?: (value: string) => void;
		/** Called when Browse button is clicked */
		onbrowse?: () => void;
		/** Called when Validate button is clicked */
		onvalidate?: () => void;
	}

	let {
		id,
		label,
		placeholder = '',
		value = '',
		hint = 'Not validated yet.',
		hintType = 'neutral',
		valid = false,
		onchange,
		onbrowse,
		onvalidate
	}: Props = $props();

	function handleInput(e: Event) {
		const target = e.target as HTMLInputElement;
		onchange?.(target.value);
	}
</script>

<div class="row">
	<div class="field">
		<label for={id}>{@render label()}</label>
		<input
			{id}
			type="text"
			{placeholder}
			{value}
			oninput={handleInput}
		/>
		<p class="hint {hintType}">{hint}</p>
	</div>
	<div class="button-group">
		<button type="button" class="secondary" onclick={() => onbrowse?.()}>Browse...</button>
		<button type="button" class="secondary" onclick={() => onvalidate?.()}>Validate</button>
	</div>
</div>

<style>
	.button-group {
		display: flex;
		gap: var(--space-2);
	}
</style>
