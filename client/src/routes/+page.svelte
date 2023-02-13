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

<!-- <script>
	import Counter from './Counter.svelte';
	import welcome from '$lib/images/svelte-welcome.webp';
	import welcome_fallback from '$lib/images/svelte-welcome.png';
	
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1>
		<span class="welcome">
			<picture>
				<source srcset={welcome} type="image/webp" />
				<img src={welcome_fallback} alt="Welcome" />
			</picture>
		</span>

		to your new<br />SvelteKit app
	</h1>

	<h2>
		try editing <strong>src/routes/+page.svelte</strong>
	</h2>

	<Counter />
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		display: block;
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		display: block;
	}
</style> -->
