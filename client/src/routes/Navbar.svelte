<script lang="ts">
    import Button from "@smui/button";
    import TopAppBar, { Row, Section, Title } from "@smui/top-app-bar";
	import { ThemeMode, user } from "../stores";
	import IconButton from '@smui/icon-button';
    import backendService from "$lib/backend-service";
	import Logo from "./Logo.svelte";

	export let isLightTheme: boolean | null;
	export let topAppBar;
</script>

<TopAppBar
	variant="fixed"
	bind:this={topAppBar}
>
	<Row>
		<Section>
			<a id="home-link" href="/">
				<Logo height="100%"/>
				<Title id="title">Background Generator</Title>
			</a>
			<Button href="/about/">About</Button>				
			{#if $user.isAuthorised()}
				<Button href="/profile/">Profile</Button>
				<Button href="/myGenerators/">My Generators</Button>
				{#if $user.getAuthorisedUserData().isAdmin}
					<Button href="/controlpanel/">Control Panel</Button>
				{/if}
			{/if}
		</Section>
		<Section align="end" toolbar>
			{#if isLightTheme === true}
				<IconButton class="material-icons"
					on:click={() => backendService.setUserPreferredTheme(ThemeMode.Dark)}
				>nightlight</IconButton>
			{:else if isLightTheme === false}
				<IconButton class="material-icons"
					on:click={() => backendService.setUserPreferredTheme(ThemeMode.Light)}
				>light_mode</IconButton>
			{/if}
			{#if $user.isGuest()}
				<Button href="/signin/">Sign In</Button>	
				<Button href="/signup/">Sign Up</Button>
			{:else if $user.isAuthorised()}
				<Button href="/signout/">Sign Out</Button>
			{:else if $user.isLoading()}
				<Button style="pointer-events: none; cursor: default;" href="/">Loading...</Button>
			{/if}
		</Section>
	</Row>
</TopAppBar>

<style>
#home-link {
	display: inline-flex;
    align-items: center;
    height: 100%;
	margin-right: 50px;
	color: inherit;
	text-decoration: none;
}
#home-link:hover {
	cursor: pointer;
}
</style>