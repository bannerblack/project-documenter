<script lang="ts">
	import type { PageProps } from './$types';
	import type { StoreType } from '@utils/tauri-store';
	import { invalidateAll } from '$app/navigation';
	import tauriStore from '@utils/tauri-store';
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import * as Card from '$lib/components/ui/card/index';
	import Checkbox from '$lib/components/ui/checkbox/checkbox.svelte';

	let { data }: PageProps = $props();

	let store: StoreType = $derived(data.store);

	let storeNames = {
		projectDirectories: {
			name: 'Project Directories',
			description: 'List of directories to scan for projects'
		},
		displayReadmeContent: {
			name: 'Display ReadMe Content',
			description: 'Whether to display ReadMe content in the project cards'
		}
	};

	let newDir = $state('');

	async function handleNewDir() {
		const dirs = (await tauriStore.get<string[]>('projectDirectories'))!;
		dirs.push(newDir);
		tauriStore.set('projectDirectories', dirs);
		newDir = '';
		await invalidateAll();
	}
</script>

<Button variant="outline" href="/">Back</Button>

<Card.Root>
	<Card.Header>
		<Card.Title>Settings</Card.Title>
		<Card.Description>Configure your application settings below.</Card.Description>
	</Card.Header>
	<Card.Content>
		<div class="settings-form">
			<h2>Project Directories</h2>

			<h3>Directories</h3>
			{#each store.projectDirectories as dir}
				<Input value={dir} />
			{/each}
			<div>Add a new Project Directory</div>
			<div class="new-dir flex flex-row items-center justify-center gap-4">
				<Input bind:value={newDir} /><Button onclick={handleNewDir}>Add</Button>
			</div>
		</div>
	</Card.Content>
</Card.Root>
