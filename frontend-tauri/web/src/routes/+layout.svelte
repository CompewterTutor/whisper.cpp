<script lang="ts">
	import '../lib/styles/theme.css';
	import '../lib/styles/base.css';
	import { theme } from '$lib/stores/theme';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<title>Careless</title>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="app">
	<header>
		<h1>Careless</h1>
		<p class="subtitle">Speech recognition powered by whisper.cpp</p>
	</header>

	<main>
		{@render children()}
	</main>
</div>
