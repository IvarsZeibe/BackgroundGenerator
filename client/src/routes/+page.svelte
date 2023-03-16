<script lang="ts">
	import { goto } from "$app/navigation";
    import Card, { Media, MediaContent, PrimaryAction } from "@smui/card";
    import { Label } from "@smui/common";
    import type { PageData } from "./$types";
    import Logo from "./Logo.svelte";

	export let data: PageData;
</script>

<svelte:head>
	<title>Background generator</title>
</svelte:head>

<div>
	<div class="hero">
		<div class="title">
			<Logo outline height="80%"/>
			<h1>Background generator</h1>
		</div>
		<p>
			Transform your website with stunning backgrounds - create unique designs in just a few clicks with our generator.
		</p>
	</div>
	{#each data.generators as generator}
		<Card class="card" >
			<PrimaryAction on:click={() => {goto(`/${generator.code}`)}}>
				<Label class="generator-name">{generator.name}</Label>
				<Media
					style="background-image: url('{generator.code}.png'); background-size: cover;"
					aspectRatio="16x9" />
			</PrimaryAction>
		</Card>
	{/each}
</div>

<style>
.title {
	display: inline-flex;
	justify-content: center;
	align-items: center;
	height: 200px;
}
.hero {
	padding: 15px 0px;
}
h1 {
	margin-left: 20px;
	font-size: 80px;
	text-align: center;
}
p {
	font-size: 1.5rem;
	font-family: sans-serif;
}
* :global(.generator-name) {
	padding: 20px 0px;
	margin: 0px;
	font-size: 50px;
	font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
	background-size: cover;
	text-align: center;
}
* :global(.card) {
	margin: 16px;
  }
</style>