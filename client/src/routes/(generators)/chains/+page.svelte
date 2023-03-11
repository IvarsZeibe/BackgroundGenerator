<script lang="ts">
	import backendService from "$lib/backend-service";
    import Color from "$lib/color";
    import { onMount } from "svelte";
    import type { LayoutData } from "../$types";
    import { context } from "../+layout.svelte";

	export let data: LayoutData;
	
	let generator = $context.generatorSettings;

	generator.width = 1920;
	generator.height = 1080;
	generator.chainCount = 1000;
	generator.circleRadius = 5;
	generator.spacing = 5;
	let isColor1Random = !$context.isPreloaded;
	let isColor2Random = !$context.isPreloaded;
	generator.backgroundColor = "#FFFFFF";
	let isSeedRandom = !$context.isPreloaded;

	$context.generatorTypeName = "chains";

	$context.generate = () => {
		generator = $context.generatorSettings;

		if (isColor1Random || !generator.color1) {
			generator.color1 = Color.getRandomHex();
		}
		if (isColor2Random || !generator.color2) {
			generator.color2 = Color.getRandomHex();
		}
		if (isSeedRandom || !generator.seed) {
			generator.seed = Math.floor(Math.random() * Math.pow(2, 32));
		}

		backendService.generate($context.generatorTypeName, generator)
		.then(blob => {
			let urlCreator = window.URL || window.webkitURL; // firefox/chrome
			$context.src = urlCreator.createObjectURL(blob);
		});
	}
</script>

<label for="width">Width</label>
<input type="number" name="width" placeholder="Width" bind:value={generator.width} /><br />
<label for="height">Height</label>
<input type="number" name="height" placeholder="Height" bind:value={generator.height} /><br />

<label for="chainCount">Chain count</label>
<input type="number" name="chainCount" bind:value={generator.chainCount} /><br />

<label for="circleRadius">Circle radius</label>
<input type="number" name="circleRadius" bind:value={generator.circleRadius} /><br />

<label for="spacing">Spacing</label>
<input type="number" name="spacing" bind:value={generator.spacing} /><br />

<label for="is_color1_random">Is random</label>
<input type="checkbox" name="is_color1_random" bind:checked={isColor1Random} />
<label for="color1">Color</label>
<input type="color" name="color1" placeholder="Color" bind:value={generator.color1} disabled={isColor1Random} /><br />

<label for="is_color2_random">Is random</label>
<input type="checkbox" name="is_color2_random" bind:checked={isColor2Random} />
<label for="color2">Color</label>
<input type="color" name="color2" placeholder="Color" bind:value={generator.color2} disabled={isColor2Random} /><br />

<label for="background">Color</label>
<input type="color" name="background" placeholder="Color" bind:value={generator.backgroundColor} /><br />

<label for="is_seed_random">Is seed random</label>
<input type="checkbox" name="is_seed_random" bind:checked={isSeedRandom} />
<label for="seed">Seed</label>
<input type="number" name="seed" placeholder="Seed" bind:value={generator.seed} disabled={isSeedRandom} /><br />