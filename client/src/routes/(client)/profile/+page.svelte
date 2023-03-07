<script lang="ts">
    import backendService from '$lib/backend-service';
    import Button from '@smui/button/src/Button.svelte';
	import Select, { Option } from '@smui/select';
    import { ThemeMode, themeMode } from '../../../stores';

	let options = [ThemeMode.UseDeviceTheme, ThemeMode.Light, ThemeMode.Dark];
	let value: ThemeMode | "loading" = "loading";

	$: if (value == "loading" && $themeMode != null) {
		value = $themeMode;
	}

	function getThemeModeName(theme: ThemeMode): string {
		switch(theme) {
			case ThemeMode.Light:
				return "Light";
			case ThemeMode.Dark:
				return "Dark";
			case ThemeMode.UseDeviceTheme:
				return "Use device theme";
		}
	}
</script>

<svelte:head>
	<title>Profile | Background generator</title>
</svelte:head>

<h1>Your profile</h1>

<h2>Preferred theme</h2>
<Select bind:value >
	{#each options as option}
        <Option value={option}>{getThemeModeName(option)}</Option>
	{/each}
	{#if value == "loading"}
		<Option value={"loading"}>Loading...</Option>
	{/if}
</Select>
<Button on:click={() => {
	if (value !== "loading") {
		backendService.setUserPreferredTheme(value);
	}
}}>Save</Button>