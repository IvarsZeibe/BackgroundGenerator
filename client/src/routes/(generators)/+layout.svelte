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

<div class="container">
	<div class="left-div">
		<div class="left-content">
			<Button on:click={$context.generate}>Generate</Button>
			<slot />
		</div>
	</div>
	<div class="right-div">
		<img src={$context.src} alt="Nothing"/>
		<div class="image-actions">
			{#if $context.src != ""}
				{#if $user.isAuthorised()}
				<Button on:click={openSaveDialog}>Save</Button>
				{:else if $user.isGuest()}
				<Button href="/signin/">Sign in to save</Button>
				{/if}
				<Button href={$context.src} download="image">Download</Button>
			{/if}
		</div>
	</div>
  </div>

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

<style>
	h1 {
		margin-left: 350px;
		text-align: center;
	}
	.container {
		width: 100%;
		max-height: 70vh;
		display: flex;
		flex-direction: row;
		flex-grow: 1;
	}

	.left-div {
		overflow-y: auto;
		width: 300px;
		padding: 20px;
	}

	.left-content {
		display: flex;
		flex-direction: column;
	}

	.right-div {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 20px;
		box-sizing: border-box;
	}

	.right-div img {
		max-width: 100%;
		max-height: 100%;
		object-fit: contain;
	}

	.image-actions {
		margin-top: auto;
		display: flex;
		flex-direction: row;
		justify-content: center;
		margin-top: 20px;
		padding: 10px 20px;
		width: 100%;
	}
	
	* :global(input[type="color"]) {
		width: 100%;
		background: none;
		border: none;
		padding: 0;
	}
	* :global(input[type="color"]:disabled) {
		filter: blur(5px);
	}
	* :global(.color-label) {
		font-family: Roboto, sans-serif;
		font-weight: 400;
		margin-left: 2px;
		font-size: 16px;
	}
	* :global(.input-group) {
		border-bottom: 1px gray solid;
		margin: 30px 0px;
	}
</style>