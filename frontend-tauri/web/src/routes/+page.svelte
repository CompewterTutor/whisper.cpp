<script lang="ts">
	import { onMount } from 'svelte';
	import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app';
	import { theme, type Theme } from '$lib/stores/theme';
	import { InputSection } from '$lib/components';
	import '$lib/types/tauri.d.ts';

	let appName = 'loading...';
	let appVersion = 'loading...';
	let tauriVersion = 'loading...';

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

	function handleThemeChange(e: Event) {
		const target = e.target as HTMLSelectElement;
		theme.set(target.value as Theme);
	}
</script>

<svelte:head>
	<title>Careless</title>
</svelte:head>

<!-- Input Files Section -->
<InputSection />

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

<!-- Theme Test Section -->
<div class="section">
	<div class="section-header">Theme Test</div>
	<div class="card">
		<div class="field">
			<label for="themeSelect">Theme</label>
			<select id="themeSelect" value={$theme} onchange={handleThemeChange}>
				<option value="system">System</option>
				<option value="light">Light</option>
				<option value="dark">Dark</option>
			</select>
		</div>
		<p class="hint neutral">Current theme: {$theme}</p>
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
