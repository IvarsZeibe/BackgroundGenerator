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

	$context.name = "";
	$context.description = "";
	let previousName = "";
	let previousDescription = "";

	function toSentanceCase(text: string) {
		return text.slice(0, 1).toUpperCase() + text.slice(1);
	}
	function openSaveDialog() {
		isDialogOpen = true;
	}
	async function save() {
		let response = await backendService.saveGenerator($context.generatorTypeName, $context.name, $context.description, $context.generatorSettings);
		if (response.ok) {
			previousName = $context.name;
			previousDescription = $context.description;
			data.id = await response.text();
		} else {
			console.log("Saving failed");
		}
	}
	function cancel() {
		$context.name = previousName;
		$context.description = previousDescription;
	}
	function saveChanges() {
		if (!data.id) {
			return;
		}
		backendService.saveGeneratorChanges($context.generatorTypeName, data.id, $context.name, $context.description, $context.generatorSettings);
		previousName = $context.name;
		previousDescription = $context.description;
	}
	onMount(async () => {
		if (data.id) {
			let generatorInfo = await backendService.getMyGenerator($context.generatorTypeName, data.id);
			$context.name = generatorInfo.name;
			$context.description = generatorInfo.description;
			previousName = $context.name;
			previousDescription = $context.description;
			$context.generatorSettings = generatorInfo.generatorSettings;
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
			<!-- <br>
			<h2>Generator settings</h2>
			{#each Object.keys($context.generatorSettings) as key}
				<span class="test">{key}:</span> {$context.generatorSettings[key]}<br>
			{/each} -->
		</div>
	</Content>
	<Actions>
		{#if data.id}
		<Button on:click={saveChanges}>
			<Label>Save changes</Label>
		</Button>
		<Button on:click={save}>
			<Label>Save as new</Label>
		</Button>
		{:else}
		<Button on:click={save}>
			<Label>Save</Label>
		</Button>
		{/if}
		<Button on:click={cancel}>
			<Label>Cancel</Label>
		</Button>
	</Actions>
</Dialog>