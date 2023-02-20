<script lang="ts" context="module">
	export const data = writable({src: "", generate: () => {}, generatorName: ""});
</script>

<script lang="ts">
    import Button from "@smui/button";
	import { onMount } from "svelte";
    import { writable } from "svelte/store";

	function toSentanceCase(text: string) {
		return text.slice(0, 1).toUpperCase() + text.slice(1);
	}

	onMount(() => $data.generate());
</script>

<h1>{toSentanceCase($data.generatorName)} generator</h1>
<button on:click={$data.generate}>Generate</button>

<slot />

<a href={$data.src} download="image">Download</a>
<img src={$data.src} alt="Nothing"/>
{#if $data.src != ""}
<Button>Save</Button>
{/if}