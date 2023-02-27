<script lang="ts">
	import backendService from "$lib/backend-service";
    import Color from "$lib/color";
    import { data } from "../+layout.svelte";

	let width = 1920;
	let height = 1080;
	let circleCount = 200;
	let maxCircleSize = 150;
	let color1: string;
	let isColor1Random = true;
	let color2: string;
	let isColor2Random = true;
	let backgroundColor: string = "#FFFFFF";
	let seed: number;
	let is_seed_random = true;

	$data.generatorName = "circles";

	$data.generate = () => {
		if (isColor1Random || !color1) {
			// generated random hex color
			color1 = Color.getRandomHex();
		}
		if (isColor2Random || !color2) {
			// generated random hex color
			color2 = Color.getRandomHex();
		}
		if (is_seed_random || !seed) {
			seed = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
		}
		let settings = {
			width, height,
			circleCount, maxCircleSize,
			color1: Color.hexToRgb(color1),
			color2: Color.hexToRgb(color2),
			backgroundColor: Color.hexToRgb(backgroundColor),
			seed
		};
		backendService.generate($data.generatorName, settings)
		.then(blob => {
			let urlCreator = window.URL || window.webkitURL; // firefox/chrome
			$data.src = urlCreator.createObjectURL(blob);
		});
	}
</script>

<label for="width">Width</label>
<input type="number" name="width" placeholder="Width" bind:value={width} /><br />
<label for="height">Height</label>
<input type="number" name="height" placeholder="Height" bind:value={height} /><br />

<label for="circle_count">Circle count</label>
<input type="number" name="circle_count" bind:value={circleCount} /><br />

<label for="max_circle_size">Max circle size</label>
<input type="number" name="max_circle_size" bind:value={maxCircleSize} /><br />

<label for="is_color1_random">Is random</label>
<input type="checkbox" name="is_color1_random" bind:checked={isColor1Random} />
<label for="color1">Color</label>
<input type="color" name="color1" placeholder="Color" bind:value={color1} disabled={isColor1Random} /><br />

<label for="is_color2_random">Is random</label>
<input type="checkbox" name="is_color2_random" bind:checked={isColor2Random} />
<label for="color2">Color</label>
<input type="color" name="color2" placeholder="Color" bind:value={color2} disabled={isColor2Random} /><br />

<label for="background">Color</label>
<input type="color" name="background" placeholder="Color" bind:value={backgroundColor} /><br />

<label for="is_seed_random">Is seed random</label>
<input type="checkbox" name="is_seed_random" bind:checked={is_seed_random} />
<label for="seed">Seed</label>
<input type="number" name="seed" placeholder="Seed" bind:value={seed} disabled={is_seed_random} /><br />