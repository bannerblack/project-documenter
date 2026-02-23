<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Card from '$lib/components/ui/card/index';
	import * as ButtonGroup from '$lib/components/ui/button-group/index';
	import Badge from '$lib/components/ui/badge/badge.svelte';

	import ReadmeContent from '@blocks/readme-content.svelte';

	import { invoke } from '@tauri-apps/api/core';
	import { openPath, openUrl } from '@tauri-apps/plugin-opener';

	import LaptopIcon from '@lucide/svelte/icons/laptop';
	import GithubIcon from '@lucide/svelte/icons/github';
	import FolderKanban from '@lucide/svelte/icons/folder-kanban';

	// ---
	let { project } = $props();

	async function openInVSCode(path: string) {
		try {
			await invoke('open_in_vscode', { path });
		} catch (error) {
			console.error('Failed to open in VS Code:', error);
		}
	}

	async function handleOpenURL(url: string) {
		try {
			await openUrl(url);
		} catch (error) {
			console.error('Failed to open URL:', error);
		}
	}

	async function handleOpenPath(path: string) {
		try {
			await openPath(path);
		} catch (error) {
			console.error('Failed to open path:', error);
		}
	}
</script>

<Card.Root class="grid-span-1 mb-4 min-w-80">
	<ButtonGroup.Root orientation="horizontal" class="pl-4">
		{#if project.github_link}
			<Button variant="outline" size="sm" onclick={() => handleOpenURL(project.github_link)}
				><GithubIcon />GitHub</Button
			>
		{/if}
		{#if project.root_folder_path}
			<Button variant="outline" size="sm" onclick={() => handleOpenPath(project.root_folder_path)}
				><LaptopIcon />Folder</Button
			>

			<Button variant="outline" size="sm" onclick={() => openInVSCode(project.root_folder_path)}
				><FolderKanban />Code</Button
			>
		{/if}
	</ButtonGroup.Root>

	<!-- Header -->
	<Card.Header class="flex flex-col justify-between gap-4">
		<Card.Title class="text-xl font-bold">{project.name}</Card.Title>
		<Card.Description>{project.description}</Card.Description>
		{#if project.tags.length > 0}
			<Card.Description class="flex flex-wrap flex-row gap-2 pt-2">
				{#each project.tags as tag}
					<Badge variant="outline">{tag}</Badge>
				{/each}
			</Card.Description>
		{/if}
	</Card.Header>

	<!-- Content -->
	<Card.Content class="text-md py-0">
		<ReadmeContent read_me_link={project.read_me_link} opener={handleOpenPath} />
	</Card.Content>
</Card.Root>
