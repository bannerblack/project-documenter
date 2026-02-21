<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { darkMode } from '$lib/stores/theme';

	let { children, data } = $props();

	$effect.pre(() => {
		document.body.classList.add('dark');
	});
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="main p-4">
	{@render children()}
</div>
