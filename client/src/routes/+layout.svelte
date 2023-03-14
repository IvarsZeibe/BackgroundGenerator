<script lang="ts">
    import { browser } from '$app/environment';
    import backendService from '$lib/backend-service';
    import { onMount } from 'svelte';
    import { ThemeMode, themeMode } from '../stores';
	import Navbar from './Navbar.svelte';
	import TopAppBar, { AutoAdjust } from '@smui/top-app-bar';
	
	let isLightThemeDevicePreferred: boolean | null;
	let themeLink: HTMLLinkElement | null;
	let isLightTheme: boolean | null;

	let topAppBar: TopAppBar;

	if (browser) {
		themeLink = document.head.querySelector<HTMLLinkElement>('#theme');
		isLightThemeDevicePreferred = window.matchMedia('(prefers-color-scheme: light)').matches;
		window.matchMedia('(prefers-color-scheme: light)').addEventListener('change', event => {
			isLightThemeDevicePreferred = event.matches;
		});
		if (themeLink?.href) {
			isLightTheme = themeLink?.href == location.origin + "/smui.css";
		} else {
			isLightTheme = isLightThemeDevicePreferred;
		}
	}

	onMount(() => {
		backendService.setProfile();
	});

	$: if (isLightThemeDevicePreferred) {
		switch($themeMode) {
			case ThemeMode.UseDeviceTheme:
				localStorage.removeItem("previousTheme");
				isLightTheme = isLightThemeDevicePreferred;
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
</svelte:head>

<div class="app">
	<Navbar bind:topAppBar bind:isLightTheme />
	<main>
		<AutoAdjust {topAppBar}>
			<slot />
		</AutoAdjust> 
	</main>	
	<footer>
		<p>visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to learn SvelteKit</p>
	</footer>
</div>

<style>
	:global(body) {
		margin: 0;
	}
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
