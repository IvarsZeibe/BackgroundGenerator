<script lang="ts">
	import { goto } from "$app/navigation";
    import type { PageData } from "./$types";

	export let data: PageData;
</script>

<svelte:head>
	<title>Background generator</title>
</svelte:head>

<div>
	<h1>Background generator</h1>
	<h2>Choose a generator:</h2>
	<ul>
		{#each data.generators as generator}
		<li>
			<button on:click={() => {goto(`/${generator.code}`)}}>{generator.name}</button>
		</li>
		{/each}
	</ul>
</div>

<style>
h1 {
	font-size: 60px;
	text-align: center;
}
ul {
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
}
li {
	list-style-type: none;
	margin: 20px;
}
li button {
	display: block;
	background-color: gray;
	width: 500px;
	height: 200px;
}
li button:hover {
	cursor: pointer;
}
</style>