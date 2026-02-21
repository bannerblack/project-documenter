<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Card from '$lib/components/ui/card/index';
	import * as ButtonGroup from '$lib/components/ui/button-group/index';
	import { invoke } from '@tauri-apps/api/core';

	import ProjectCard from '@blocks/project-card.svelte';

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
</script>

<Button variant="outline" href="/settings">Settings</Button>

<pre>{JSON.stringify(data.projects, null, 2)}</pre>
<pre>{JSON.stringify(data.store, null, 2)}</pre>

<div class="flex gap-4">
	{#each [0, 1] as col}
		<div class="flex flex-1 flex-col gap-4">
			{#each projects.filter((_, i) => i % 2 === col) as project}
				<ProjectCard {project} />
			{/each}
		</div>
	{/each}
</div>
