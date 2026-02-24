<script lang="ts">
	import { onMount } from 'svelte';
	import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app';

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
</script>

<svelte:head>
	<title>Careless</title>
</svelte:head>

<main>
	<h1>SvelteKit + Tauri</h1>
	<p>Frontend refactor successful!</p>

	<div class="info-panel">
		<h2>App Info</h2>
		<dl>
			<dt>App Name</dt>
			<dd>{appName}</dd>
			<dt>App Version</dt>
			<dd>{appVersion}</dd>
			<dt>Tauri Version</dt>
			<dd>{tauriVersion}</dd>
		</dl>
	</div>
</main>

<style>
	main {
		max-width: 600px;
		margin: 2rem auto;
		padding: 1rem;
		font-family: system-ui, -apple-system, sans-serif;
	}

	h1 {
		color: #ff3e00;
	}

	.info-panel {
		background: #f5f5f5;
		padding: 1rem;
		border-radius: 8px;
		margin-top: 1rem;
	}

	dl {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.5rem;
	}

	dt {
		font-weight: bold;
	}

	dd {
		margin: 0;
	}
</style>
