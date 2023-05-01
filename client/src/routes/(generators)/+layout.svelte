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
    import { page } from "$app/stores";
    import { goto } from "$app/navigation";

	export let data: LayoutData;
	
	$context.isPreloaded = !!data.id;
	let isDialogOpen = false;

	$context.name = "";
	$context.description = "";
	let previousName = "";
	let previousDescription = "";
	let errorMessage = "";

	function toSentanceCase(text: string) {
		return text.slice(0, 1).toUpperCase() + text.slice(1);
	}
	function openSaveDialog() {
		errorMessage = "";
		isDialogOpen = true;
	}
	async function handleSave(e: CustomEvent<any>) {
		e.stopPropagation();
		let response = await backendService.saveGenerator($context.generatorTypeName, $context.name, $context.description, $context.generatorSettings);
		if (response.ok) {
			previousName = $context.name;
			previousDescription = $context.description;
			data.id = await response.text();
			let query = $page.url.searchParams;
			query.set("id", data.id);
			goto(`?${query.toString()}`);
			isDialogOpen = false;
		} else {
			if (response.status == 400) {
				let message = await response.text();
				if (message != '') {
					errorMessage = `Saving failed (${message})`;
					return;
				}
			}
			errorMessage = `Saving failed`;
		}
	}
	function cancel() {
		$context.name = previousName;
		$context.description = previousDescription;
	}
	async function saveChanges(e: CustomEvent<any>) {
		e.stopPropagation();
		if (!data.id) {
			return;
		}
		let response = await backendService.saveGeneratorChanges($context.generatorTypeName, data.id, $context.name, $context.description, $context.generatorSettings);
		if (response.ok) {
			previousName = $context.name;
			previousDescription = $context.description;
			isDialogOpen = false;
		} else {
			errorMessage = `Saving failed`;
		}
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
			</Textfield>
			<br>
			<Textfield
				bind:value={$context.description}
				label="Description" type="text"
			>
			</Textfield>
			<br>
			<Label style="color: #b71c1c">{errorMessage}</Label>
		</div>
	</Content>
	<Actions>
		{#if data.id}
		<Button on:click={saveChanges}>
			<Label>Save changes</Label>
		</Button>
		<Button on:click={handleSave}>
			<Label>Save as new</Label>
		</Button>
		{:else}
		<Button on:click={handleSave}>
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