<script lang="ts">
	import backendService from "$lib/backend-service";
	import { onMount } from "svelte";

	let level_of_detail = 4;

	let src = "";

	let generatorName: string = "colorful";

	function toSentanceCase(text: string) {
		return text.slice(0, 1).toUpperCase() + text.slice(1);
	}

	function generate() {
		let settings = {level_of_detail: level_of_detail};
		backendService.generate(generatorName, settings)
		.then(blob => {
			let urlCreator = window.URL || window.webkitURL; // firefox/chrome
			src = urlCreator.createObjectURL(blob);
		});
	}

	onMount(generate);
</script>

<h1>{toSentanceCase(generatorName)} generator</h1>
<button on:click={generate}>Generate</button>

<slot />

<a href={src} download="image">Download</a>
<img {src} alt="Nothing"/>