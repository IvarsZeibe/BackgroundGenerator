<script lang="ts">
    import { browser } from '$app/environment';
    import backendService from '$lib/backend-service';
    import Button from '@smui/button/src/Button.svelte';
    import { onMount } from 'svelte';
    import { ThemeMode, themeMode } from '../stores';
	import Navbar from './Navbar.svelte';
	
	let devicePreferredTheme: string | null;
	let themeLink: HTMLLinkElement | null;
	let isLightTheme: boolean | null;

	if (browser) {
		themeLink = document.head.querySelector<HTMLLinkElement>('#theme');
		devicePreferredTheme = window.matchMedia('(prefers-color-scheme: dark)').matches ? "dark" : "light";
		window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', event => {
			devicePreferredTheme = event.matches ? "dark" : "light";
		});
	}

	onMount(() => {
		backendService.setProfile();
	});

	$: if (devicePreferredTheme) {
		switch($themeMode) {
			case ThemeMode.UseDeviceTheme:
				localStorage.removeItem("previousTheme");
				isLightTheme = devicePreferredTheme == "light";
				break;
			case ThemeMode.Dark:
				isLightTheme = false;
				break;
			case ThemeMode.Light:
				isLightTheme = true;
				break;
		}
	}

	$: if (themeLink) {
		if (isLightTheme) {
			themeLink.href = "/smui.css";
			if ($themeMode != ThemeMode.UseDeviceTheme) {
				localStorage.setItem("previousTheme", "light");
			}
		} else if (isLightTheme === false) {
			themeLink.href = "/smui-dark.css";
			if ($themeMode != ThemeMode.UseDeviceTheme) {
				localStorage.setItem("previousTheme", "dark");
			}
		}
	}	
</script>

<svelte:head>
	<title>Background generator</title>
	<meta name="description" content="Background generator" />
	
	<script>
		let preferredTheme = localStorage.getItem("previousTheme");
		if (preferredTheme) {
			let themeLink = document.head.querySelector('#theme');
			if (preferredTheme == "light") {
				themeLink.href = "/smui.css"
			} else if (preferredTheme == "dark") {
				themeLink.href = "/smui-dark.css"
			}
		}
	</script>
</svelte:head>

<Button on:click={() => {backendService.setUserPreferredTheme(ThemeMode.Light)}}>Light</Button>
<Button on:click={() => {backendService.setUserPreferredTheme(ThemeMode.Dark)}}>Dark</Button>
<Button on:click={() => {backendService.setUserPreferredTheme(ThemeMode.UseDeviceTheme)}}>Use device theme</Button>
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
