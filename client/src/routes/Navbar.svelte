<script lang="ts">
    import Button from "@smui/button";
	import { user } from "../stores";
</script>

<div id="navbar">
	<ul class="mdc-theme-surface">
		<li><Button href="/">Home</Button></li>
		<li><Button href="/about/">About</Button></li>
		{#if $user.isGuest()}
		<li><Button href="/signin/">Sign In</Button></li>
		<li><Button href="/signup/">Sign Up</Button></li>
		{:else if $user.isAuthorised()}
		<li><Button href="/profile/">Profile</Button></li>
			{#if $user.getAuthorisedUserData().isAdmin}
			<li><Button href="/controlpanel/">Control Panel</Button></li>
			{/if}
		<li><Button href="/signout/">Sign Out</Button></li>
		{:else}
		<li><Button style="pointer-events: none; cursor: default;" href="/">Loading...</Button></li>
		{/if}
	</ul>
</div>

<style>
ul {
	list-style-type: none;
	margin: 0;
	padding: 0;
	overflow: hidden;
	background-color: var(--mdc-theme-surface);
}

li {
	float: left;
}

a {
	display: block;
	text-align: center;
	padding: 14px 16px;
	text-decoration: none;
}
</style>