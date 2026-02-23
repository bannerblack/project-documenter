<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import EmptyProjects from '@blocks/empty-projects.svelte';
	import * as Card from '$lib/components/ui/card/index';
	import * as ButtonGroup from '$lib/components/ui/button-group/index';
	import { invoke } from '@tauri-apps/api/core';

	import type { Projects } from '@utils/types';

	import ProjectCard from '@blocks/project-card.svelte';

	//---------------------------------------------

	let { data } = $props();

	let projects : Projects[] = $derived(data.projects);

	// ---------------------------------------------
</script>

<pre>{JSON.stringify(data.projects, null, 2)}</pre>
<pre>{JSON.stringify(data.store, null, 2)}</pre>

{#if projects.length === 0}
	<EmptyProjects />
{:else}
	<div class="flex gap-4">
		{#each [0, 1] as col}
			<div class="flex flex-1 flex-col gap-4">
				{#each projects.filter((_, i) => i % 2 === col) as project}
					<ProjectCard {project} />
				{/each}
			</div>
		{/each}
	</div>
{/if}
