<script lang="ts">
	import Card, {
		Content,
		PrimaryAction,
		Media,
		MediaContent,
        Actions,
        ActionButtons,
	} from '@smui/card';
    import { goto } from "$app/navigation";
    import Button, { Label } from '@smui/button';
    import { onMount } from 'svelte';
    import backendService, { type GeneratorDescription } from '$lib/backend-service';
    import Dialog, { Actions as DialogActions, Content as DialogContent, Title as DialogTitle } from "@smui/dialog";
    import { Title } from '@smui/dialog';

	let isDeleteDialogOpen = false;
	let deleteAction = () => {};

	onMount(async () => {
		generators = await backendService.getMyGenerators();
	})

	let generators: GeneratorDescription[] = [];

	function openGenerator(generator: GeneratorDescription) {
		goto(`/${generator.generatorTypeCode}/?id=${generator.id}`)
	}
	function openDeleteDialog(generator: GeneratorDescription) {
		isDeleteDialogOpen = true;
		deleteAction = () => {
			backendService.deleteGenerator(generator.generatorTypeCode, generator.id);
			let index = generators.indexOf(generator);
			if (index > -1) {
				generators.splice(index, 1);
				generators = generators;
			}
		};
	}
	
</script>

<h1>My generators</h1>
<div>
<div class="card-display">
	{#each generators as generator}
	<div class="card-container">
		<Card style="min-width: 380px">
			<PrimaryAction on:click={() => openGenerator(generator)}>
				<div style="padding: 1rem;">
					<h2 class="mdc-typography--headline6" style="margin: 0;">
						{generator.name}
					</h2>
					<h3 class="mdc-typography--subtitle2" style="margin: 0; color: #888;">
						Created on {generator.dateCreated.toLocaleString("en-UK",
						{ year: 'numeric', month: '2-digit', day: '2-digit',
						hour: "2-digit", minute: "2-digit", hour12: false })}
					</h3>
					<h3 class="mdc-typography--subtitle2" style="margin: 0; color: #888;">
						Type: {generator.generatorType}
					</h3>
				</div>
				<Media class="card-media-16x9" aspectRatio="16x9" />
				<Content class="mdc-typography--body2">
					{generator.description}
				</Content>
			</PrimaryAction>
			<Button on:click={() => openDeleteDialog(generator)}>
				<Label>Delete</Label>
			</Button>
		</Card>
	</div>
	{/each}
</div>	
</div>

<Dialog
	bind:open={isDeleteDialogOpen}
>
	<!-- Title cannot contain leading whitespace due to mdc-typography-baseline-top() -->
	<DialogTitle>Are you sure you want to delete</DialogTitle>
	<DialogContent>This action is irreversible.</DialogContent>
	<DialogActions>
		<Button>
		<Label>No</Label>
		</Button>
		<Button on:click={deleteAction}>
		<Label>Yes</Label>
		</Button>
	</DialogActions>
</Dialog>

<style>
	.card-display {
		display: flex;
		flex-wrap: wrap;
		justify-content: stretch;
		justify-content: center;
		background-color: var(--mdc-theme-background, #f8f8f8);
		border: 1px solid
		var(--mdc-theme-text-hint-on-background, rgba(0, 0, 0, 0.1));
	}
	* :global(.card-container) {
		display: inline-flex;
		/* justify-content: center; */
		align-items: center;
		min-height: 200px;
		width: 380px;
		max-width: 100%;
		overflow-x: auto;
		/* background-color: var(--mdc-theme-background, #f8f8f8); */
		/* border: 1px solid */
		/* var(--mdc-theme-text-hint-on-background, rgba(0, 0, 0, 0.1)); */
		padding: 20px;
		margin-right: 20px;
		margin-bottom: 20px;
	}
	* :global(.card-container > *) {
		margin: 0 auto;
	}
	@media (max-width: 480px) {
		* :global(.card-container) {
		margin-right: 0;
		padding-right: 0;
		padding-left: 0;
		}
	}
	* :global(.card-media-16x9) {
	  background-image: url(https://place-hold.it/320x180?text=16x9&fontsize=23);
	}
  </style>