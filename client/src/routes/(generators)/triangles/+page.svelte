<script lang="ts">
	import backendService from "$lib/backend-service";
    import Color from "$lib/color";
    import { Label } from "@smui/common";
    import Select, { Option } from "@smui/select";
    import Switch from "@smui/switch";
    import Textfield from "@smui/textfield";
    import { context } from "../+layout.svelte";

	let generator = $context.generatorSettings;

	generator.width = 1920;
	generator.height = 1080;
	generator.edgeCount = 15;
	generator.seed = 0;
	let isColor1Random = !$context.isPreloaded;
	let isColor2Random = !$context.isPreloaded;
	let isSeedRandom = !$context.isPreloaded;
	let modeAsString = generator.mode ? generator.mode.toString() : "1";
	$: generator.mode = parseInt(modeAsString);

	$context.generatorTypeName = "triangles";

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

<Textfield type="number" label="Width" bind:value={generator.width} />
<Textfield type="number" label="Height" bind:value={generator.height} />
<Textfield type="number" label="Edge count" bind:value={generator.edgeCount} />

<Select label="Mode" bind:value={modeAsString}>
	<Option value=1>Quad</Option>
	<Option value=2>Diagonal</Option>
</Select>

<div class="input-group">
	<Label class="color-label">Accent Color 1</Label>
	<input bind:value={generator.color1} type="color" disabled={isColor1Random} />
	<Switch bind:checked={isColor1Random}></Switch>
	<Label class="color-label">Is Random</Label>
</div>

<div class="input-group">
	<Label class="color-label">Accent Color 2</Label>
	<input bind:value={generator.color2} type="color" disabled={isColor2Random} />
	<Switch bind:checked={isColor2Random}></Switch>
	<Label class="color-label">Is Random</Label>
</div>

<Textfield type="number" label="Seed" bind:value={generator.seed} disabled={isSeedRandom} />
<div>
	<Switch bind:checked={isSeedRandom}></Switch>
	<Label class="color-label">Is Random</Label>
</div>
