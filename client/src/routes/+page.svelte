<script lang="ts">
	import { goto } from "$app/navigation";

	let generators = [{ name: "triangle" }, { name: "colorful" }];
</script>

<svelte:head>
	<title>Background generator</title>
	<meta name="description" content="Background generator" />
</svelte:head>

<div>
	<h1>Background generator</h1>
	<h2>Choose a generator:</h2>
	<ul>
		{#each generators as generator}
		<li>
			<button on:click={() => {goto(`/${generator.name}`)}}>{generator.name}</button>
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