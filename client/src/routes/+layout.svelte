<script lang="ts">
    import backendService from '$lib/backend-service';
    import Button from '@smui/button/src/Button.svelte';
    import { onMount } from 'svelte';
    import { ThemeMode, themeMode } from '../stores';
	import Navbar from './Navbar.svelte';
	
	let theme: any = null;
	let isLightTheme: boolean | undefined;
	let preferredTheme: string | undefined;

	onMount(() => {
		theme = document.getElementById("theme");
		preferredTheme = window.matchMedia('(prefers-color-scheme: dark)').matches ? "dark" : "light";
		window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', event => {
			preferredTheme = event.matches ? "dark" : "light";
		});

		backendService.setProfile();
	})
	$: {
		if (preferredTheme) {
			switch($themeMode) {
				case ThemeMode.Auto:
					console.log(preferredTheme);
					isLightTheme = preferredTheme == "light";
					console.log(preferredTheme);
					break;
				case ThemeMode.Dark:
					isLightTheme = false;
					break;
				case ThemeMode.Light:
					isLightTheme = true;
					break;
			}
		}
	} 
	$: {
		if (isLightTheme) {
			if (theme.href != window.location.origin + "/smui.css") {
				theme.href = "/smui.css";
			}
			localStorage.setItem("previousTheme", "light");
		} else if (isLightTheme === false) {
			if (theme.href != window.location.origin + "/smui-dark.css") {
				theme.href = "/smui-dark.css";
			}
			localStorage.setItem("previousTheme", "dark");
		}
	}
</script>
<svelte:head>
	<title>Background generator</title>
	<meta name="description" content="Background generator" />
	
	<script>
		// Determines theme from previously used one
		let preferredTheme = localStorage.getItem("previousTheme");
		let theme = document.getElementById("theme");
		if (preferredTheme == "light") {
			theme.href = "/smui.css"
		} else if (preferredTheme == "dark") {
			theme.href = "/smui-dark.css"
		}
		console.log("s")
	</script>
</svelte:head>

<Button on:click={() => {$themeMode = ThemeMode.Light}}>Light</Button>
<Button on:click={() => {$themeMode = ThemeMode.Dark}}>Dark</Button>
<Button on:click={() => {$themeMode = ThemeMode.Auto}}>Auto</Button>
<div class="app">
	<Navbar />
	<main>
		<slot />
	</main>

	<footer>
		<p>visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to learn SvelteKit</p>
	</footer>
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 1rem;
		width: 100%;
		max-width: 64rem;
		margin: 0 auto;
		box-sizing: border-box;
	}

	footer {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 12px;
	}

	footer a {
		font-weight: bold;
	}

	@media (min-width: 480px) {
		footer {
			padding: 12px 0;
		}
	}
</style>
