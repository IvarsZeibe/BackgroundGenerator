<script lang="ts">
	import { user } from "../stores";
</script>

<div id="navbar">
	<ul>
		<li><a href="/">Home</a></li>
		<li><a href="/about/">About</a></li>
		{#if $user.isGuest()}
		<li><a href="/signin/">Sign In</a></li>
		<li><a href="/signup/">Sign Up</a></li>
		{:else if $user.isAuthorised()}
		<li><a href="/profile/">Profile</a></li>
			{#if $user.getAuthorisedUserData().isAdmin}
			<li><a href="/controlpanel/">Control Panel</a></li>
			{/if}
		<li><a href="/signout/">Sign Out</a></li>
		{:else}
		<li><a style="pointer-events: none; cursor: default;" href="/">Loading...</a></li>
		{/if}
	</ul>
</div>

<style>
ul {
	list-style-type: none;
	margin: 0;
	padding: 0;
	overflow: hidden;
	background-color: #333;
}

li {
	float: left;
}

a {
	display: block;
	color: white;
	text-align: center;
	padding: 14px 16px;
	text-decoration: none;
}

a:hover {
	background-color: #111;
}
</style>