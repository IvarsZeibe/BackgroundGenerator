<script lang="ts">
	import backendService from "$lib/backend-service";
    import { data } from "../+layout.svelte";

	let levelOfDetail = 4;

	$data.generatorName = "colorful";

	$data.generate = () => {
		let settings = { levelOfDetail };
		backendService.generate($data.generatorName, settings)
		.then(blob => {
			let urlCreator = window.URL || window.webkitURL; // firefox/chrome
			$data.src = urlCreator.createObjectURL(blob);
		});
	}
</script>

<label for="level_of_detail">Level of detail</label>
<input type="number" name="level_of_detail" placeholder="level_of_detail" bind:value={levelOfDetail} /><br />