<script lang="ts">
	import backendService from "$lib/backend-service";
    import Color from "$lib/color";
    import { data } from "../+layout.svelte";

	let width = 1920;
	let height = 1080;
	let edgeCount = 15;
	let color1: string;
	let isColor1Random = true;
	let color2: string;
	let isColor2Random = true;
	let seed: number;
	let isSeedRandom = true;
	let mode = 0;

	$data.generatorName = "triangle";

	$data.generate = () => {
		if (isColor1Random || !color1) {
			// generated random hex color
			color1 = Color.getRandomHex();
		}
		if (isColor2Random || !color2) {
			// generated random hex color
			color2 = Color.getRandomHex();
		}
		if (isSeedRandom || !seed) {
			seed = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
		}
		let settings = {
			width, height, edgeCount,
			color1: Color.hexToRgb(color1),
			color2: Color.hexToRgb(color2),
			seed, mode
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
<label for="edge_count">Edge count</label>
<input type="number" name="edge_count" placeholder="Edge_count" bind:value={edgeCount} /><br />

<label for="is_color1_random">Is random</label>
<input type="checkbox" name="is_color1_random" bind:checked={isColor1Random} />
<label for="color1">Color</label>
<input type="color" name="color1" placeholder="Color" bind:value={color1} disabled={isColor1Random} /><br />

<label for="is_color2_random">Is random</label>
<input type="checkbox" name="is_color2_random" bind:checked={isColor2Random} />
<label for="color2">Color</label>
<input type="color" name="color2" placeholder="Color" bind:value={color2}  disabled={isColor2Random} /><br />


<label for="is_seed_random">Is seed random</label>
<input type="checkbox" name="is_seed_random" bind:checked={isSeedRandom} />
<label for="seed">Seed</label>
<input type="number" name="seed" placeholder="Seed" bind:value={seed} disabled={isSeedRandom} /><br />

<label for="mode">Choose a mode:</label>
<select name="mode" bind:value={mode}>
	<option value={0}>Quad</option>
	<option value={1}>Diagonal</option>
</select>