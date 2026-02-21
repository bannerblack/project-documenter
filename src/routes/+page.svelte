<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Card from '$lib/components/ui/card/index';
	import * as ButtonGroup from '$lib/components/ui/button-group/index';
	import { invoke } from '@tauri-apps/api/core';
	import { openPath, openUrl } from '@tauri-apps/plugin-opener';
	import ReadmeContent from './readme-content.svelte';

	type Projects = {
		id: number;
		title: string;
		description: string;
		local_link: string;
		git_link: string;
		readme_link: string;
		related_projects?: number[];
		readme_content?: string;
	};

	//---------------------------------------------

	let { data } = $props();

	let projects = $derived(data.projects);

	// ---------------------------------------------

	async function openInVSCode(path: string) {
		try {
			await invoke('open_in_vscode', { path });
		} catch (error) {
			console.error('Failed to open in VS Code:', error);
		}
	}
</script>

<div class="grid grid-cols-2 gap-4">
	{#each projects as project}
		<Card.Root class="mb-4 grid-span-1 min-w-80">
			<ButtonGroup.Root orientation="horizontal" class="pl-4">
				<Button variant="outline" size="sm" onclick={() => openUrl(project.git_link)}
					>Open GitHub</Button
				>
				<Button variant="outline" size="sm" onclick={() => openPath(project.readme_link)}
					>Open Readme</Button
				>
				<Button variant="outline" size="sm" onclick={() => openInVSCode(project.local_link)}
					>Open Local</Button
				>
				<Button variant="outline" size="sm" onclick={() => openInVSCode(project.local_link)}
					>Open in Code</Button
				>
			</ButtonGroup.Root>

			<Card.Header class="flex flex-col justify-between gap-4">
				<Card.Title>{project.title}</Card.Title>
				<Card.Description>{project.description}</Card.Description>
			</Card.Header>

			<Card.Content class="text-md py-0">
				<ReadmeContent readme_link={project.readme_link} />
			</Card.Content>
		</Card.Root>
	{/each}
</div>
