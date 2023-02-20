<script lang="ts">
	import backendService from "$lib/backend-service";
    import { data } from "../+layout.svelte";

	let level_of_detail = 4;

	$data.generatorName = "colorful";

	$data.generate = () => {
		let settings = {level_of_detail: level_of_detail};
		backendService.generate($data.generatorName, settings)
		.then(blob => {
			let urlCreator = window.URL || window.webkitURL; // firefox/chrome
			$data.src = urlCreator.createObjectURL(blob);
		});
	}
</script>

<label for="level_of_detail">Level of detail</label>
<input type="number" name="level_of_detail" placeholder="level_of_detail" bind:value={level_of_detail} /><br />