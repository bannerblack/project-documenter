<script lang="ts">
	// Display readme content for each project
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { marked } from 'marked';
	import * as Accordion from '$lib/components/ui/accordion/index.js';
	import Button from '$lib/components/ui/button/button.svelte';
	import LaptopIcon from '@lucide/svelte/icons/laptop';

	let content = $state<string | null>(null);
	let error: string | null = $state(null);

	let { read_me_link, opener } = $props();

	$effect(() => {
		(async () => {
			try {
				let raw = await invoke<string>('get_readme_content', { path: read_me_link });

				const maxLength = 300;

				if (raw.length > maxLength) {
					raw = raw.substring(0, maxLength) + '...';
				}

				// Parse markdown to HTML
				let html = await marked.parse(raw);
				content = html;
			} catch (e) {
				error = e instanceof Error ? e.message : String(e);
			}
		})();
	});
</script>

<div class="readme mt-2 border-t">
	{#if error}
		<p class="text-red-600">Error loading README content: {error}</p>
	{:else if content}
		<Accordion.Root type="single">
			<Accordion.Item value="readme-preview">
				<Accordion.Trigger>Show ReadMe</Accordion.Trigger>
				<Accordion.Content>
					<div class="readme mb-2 flex flex-col gap-6 rounded-md border p-6">{@html content}</div>
					<Button size="sm" variant="outline" onclick={() => opener(read_me_link)}
						>Open ReadMe</Button
					>
				</Accordion.Content>
			</Accordion.Item>
		</Accordion.Root>
	{:else}{/if}
</div>

<style postcss>
	@reference 'tailwindcss';

	.readme {
		:global(img) {
			max-width: 100%;
			height: auto;
		}

		:global(h1) {
			@apply text-4xl font-bold;
			color: var(--color-h1);
		}

		:global(h2) {
			font-size: 1.5em;
			margin-bottom: 0.5em;
			color: var(--color-h2);
		}

		:global(h3) {
			font-size: 1.25em;
			margin-bottom: 0.5em;
			color: var(--color-h3);
		}

		:global(h4) {
			font-size: 1.1em;
			margin-bottom: 0.5em;
			color: var(--color-h4);
		}

		:global(h5) {
			font-size: 1em;
			margin-bottom: 0.5em;
			color: var(--color-h5);
		}

		:global(h6) {
			font-size: 0.9em;
			margin-bottom: 0.5em;
			color: var(--color-h6);
		}

		:global(pre) {
			padding: 1em;
			overflow-x: auto;
			border-radius: 5px;
			background-color: var(--color-pre-bg);
			color: var(--color-pre-text);
		}
	}
</style>
