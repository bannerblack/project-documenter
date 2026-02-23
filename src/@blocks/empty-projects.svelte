<script lang="ts">
	import * as Card from '$lib/components/ui/card/index';
	import * as Accordion from '$lib/components/ui/accordion/index';
</script>

<Card.Root>
	<Card.Header>
		<Card.Title class="text-center text-2xl font-semibold">No Projects Found</Card.Title>
		<Card.Description class="text-center"
			>No projects are available in the configured directories.</Card.Description
		>
	</Card.Header>

	<Card.Content >
		<Accordion.Root type="single">
			<Accordion.Item>
				<Accordion.Trigger>Instructions</Accordion.Trigger>
				<Accordion.Content class="space-y-4">
					<p>
						In order to see projects, go to settings and add a project directory. This folder should
						contain project directories as direct children.
					</p>
					<p>
						Once you've added a project directory, add the following to the project README.md file
						like so:
					</p>
					<p>
						&lt;!-- Title: Example Project Name --&gt;<br />
						&lt;!-- Description: A brief description of the project. --&gt;<br />
						&lt;!-- Github: https://github.com/example/example-project --&gt;<br />
					</p>
					<p>Don't copy and paste this, though. The title is the only mandatory field.</p>
				</Accordion.Content>
			</Accordion.Item>
		</Accordion.Root>
	</Card.Content>
</Card.Root>
