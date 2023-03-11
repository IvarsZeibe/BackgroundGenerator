<script lang="ts" context="module">
	export const context: Writable<{
		src: string,
		generate: () => void,
		generatorTypeName: string,
		name: string,
		description: string
		generatorSettings: { [key: string]: any},
		save: () => void,
		isPreloaded: boolean
	}> = writable({
		src: "",
		generate: () => {},
		name: "",
		description: "",
		generatorTypeName: "",
		generatorSettings: {},
		save: () => {},
		isPreloaded: false
	});
</script>

<script lang="ts">
    import backendService from "$lib/backend-service";
    import Button, { Label } from "@smui/button";
    import Dialog, { Actions, Content, Title } from "@smui/dialog";
    import Textfield from "@smui/textfield";
	import { onMount } from "svelte";
    import { writable, type Writable } from "svelte/store";
    import { user } from "../../stores";
    import type { LayoutData } from "./$types";

	export let data: LayoutData;
	
	$context.isPreloaded = !!data.id;
	let isDialogOpen = false;

	function toSentanceCase(text: string) {
		return text.slice(0, 1).toUpperCase() + text.slice(1);
	}
	function openSaveDialog() {
		isDialogOpen = true;
	}
	function save() {
		backendService.saveGenerator($context.generatorTypeName, $context.name, $context.description, $context.generatorSettings);
	}
	onMount(async () => {
		if (data.id) {
			$context.generatorSettings = await backendService.getMyGenerator($context.generatorTypeName, data.id);
		}
		$context.generate();
	});
</script>

<h1>{toSentanceCase($context.generatorTypeName)} generator</h1>
<button on:click={$context.generate}>Generate</button>

<slot />
<a href={$context.src} download="image">Download</a>
<img src={$context.src} alt="Nothing"/>
{#if $context.src != ""}
	{#if $user.isAuthorised()}
	<Button on:click={openSaveDialog}>Save</Button>
	{:else if $user.isGuest()}
	<Button href="/signin/">Sign in to save</Button>
	{/if}
{/if}

<Dialog
	bind:open={isDialogOpen}
	scrimClickAction=""
	escapeKeyAction=""
	aria-labelledby="mandatory-title"
	aria-describedby="mandatory-content"
>
	<Title id="mandatory-title">Save generator</Title>
	<Content id="mandatory-content">
		<div>
			<Textfield
				bind:value={$context.name}
				label="Name" type="text"
			>
				<!-- <HelperText validationMsg slot="helper">{dialogData.id.errorMessage}</HelperText> -->
			</Textfield>
			<br>
			<Textfield
				bind:value={$context.description}
				label="Description" type="text"
			>
				<!-- <HelperText validationMsg slot="helper">{dialogData.id.errorMessage}</HelperText> -->
			</Textfield>
			<br>
			<Label>Generator settings</Label><br>
			{#each Object.keys($context.generatorSettings) as key}
				{key}: {$context.generatorSettings[key]}<br>
			{/each}
		</div>
	</Content>
	<Actions>
		<Button on:click={save}>
			<Label>Save</Label>
		</Button>
		<Button>
			<Label>Cancel</Label>
		</Button>
	</Actions>
</Dialog>
