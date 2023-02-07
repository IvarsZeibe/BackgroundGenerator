<script lang="ts">
    import { onMount } from "svelte";
    import backendService from "../services/backend-service";

    let width = 500;
    let height = 500;
    let edge_count = 10;
    let color1: string;
    let is_color1_random = true;
    let color2: string;
    let is_color2_random = true;
    let seed;
    let is_seed_random = true;
    let mode = 0;
    let level_of_detail = 4;

    let src = "";

    export let currentRoute;
    const params = {}
    let generatorName: string = currentRoute.queryParams["name"];

    function toSentanceCase(text: string) {
        return text.slice(0, 1).toUpperCase() + text.slice(1);
    }
    // let settings = {}

    function generate() {
        let settings = {};
        if (generatorName == "triangle") {
            if (is_color1_random || !color1) {
                // generated random hex color
                color1 = '#'+(Math.random() * 0xFFFFFF << 0).toString(16).padStart(6, '0');
            }
            if (is_color2_random || !color2) {
                // generated random hex color
                color2 = '#'+(Math.random() * 0xFFFFFF << 0).toString(16).padStart(6, '0');
            }
            if (is_seed_random || !seed) {
                seed = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
            }
            // if (seed) {
            //     settings['seed'] = seed;
            // }
            settings = {width: width, height: height, edge_count: edge_count, color1: color1, color2: color2, seed: seed, mode: mode};
        } else if (generatorName == "colorful") {
            settings = {level_of_detail: level_of_detail};
        }
        backendService.generate(generatorName, settings)
        .then(blob => {
            let urlCreator = window.URL || window.webkitURL; // firefox/chrome
            src = urlCreator.createObjectURL(blob);
            // src = img.toString()
        });
    }

    onMount(generate);
</script>

<h1>{toSentanceCase(generatorName)} generator</h1>
<button on:click={generate}>Generate</button>

{#if generatorName == "triangle"}
<label for="width">Width</label>
<input type="number" name="width" placeholder="Width" bind:value={width} /><br />
<label for="height">Height</label>
<input type="number" name="height" placeholder="Height" bind:value={height} /><br />
<label for="edge_count">Edge count</label>
<input type="number" name="edge_count" placeholder="Edge_count" bind:value={edge_count} /><br />

<label for="is_color1_random">Is random</label>
<input type="checkbox" name="is_color1_random" bind:checked={is_color1_random} />
<label for="color1">Color</label>
<input type="color" name="color1" placeholder="Color" bind:value={color1} disabled={is_color1_random} /><br />

<label for="is_color2_random">Is random</label>
<input type="checkbox" name="is_color2_random" bind:checked={is_color2_random} />
<label for="color2">Color</label>
<input type="color" name="color2" placeholder="Color" bind:value={color2}  disabled={is_color2_random} /><br />


<label for="is_seed_random">Is seed random</label>
<input type="checkbox" name="is_seed_random" bind:checked={is_seed_random} />
<label for="seed">Seed</label>
<input type="number" name="seed" placeholder="Seed" bind:value={seed} disabled={is_seed_random} /><br />

<label for="mode">Choose a mode:</label>
<select name="mode" bind:value={mode}>
    <option value={0}>Quad</option>
    <option value={1}>Diognal</option>
</select>
{:else if generatorName == "colorful" }

<label for="level_of_detail">Level of detail</label>
<input type="number" name="level_of_detail" placeholder="level_of_detail" bind:value={level_of_detail} /><br />
{/if}
<a href={src} download="image">Download</a>
<img {src} alt="Nothing"/>