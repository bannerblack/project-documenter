<script lang="ts">
	import { page } from '$app/state';
	import * as ButtonGroup from '$lib/components/ui/button-group/index';
	import Button from '$lib/components/ui/button/button.svelte';

	const routes = [
		{ path: '/', label: 'Projects' },
		{ path: '/files-folders', label: 'Folders & Files' },
		{ path: '/settings', label: 'Settings' },
	];
</script>

<ButtonGroup.Root orientation="horizontal">
	{#each routes as route (route.path)}
		<Button
			href={route.path}
			variant="outline"
			class={page.url.pathname === route.path ? 'active' : ''}
		>
			{route.label}
		</Button>
	{/each}
</ButtonGroup.Root>

<style>
	:global(.active) {
		background-color: var(--muted);
	}
</style>