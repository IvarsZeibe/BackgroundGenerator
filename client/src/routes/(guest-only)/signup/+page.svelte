<script lang="ts">
	import backendService from '$lib/backend-service';

	let email = '';
	let password = '';

	async function handleSubmit(e: { preventDefault: () => void; }) {
		e.preventDefault();
		let { isOk, message } = await backendService.register(email, password);
		console.log("Is ok: " + isOk);
		console.log("Message: " + message);
	}
</script>

<svelte:head>
	<title>Sign up | Background generator</title>
</svelte:head>

<h1>Sign up</h1>
<div>
	<form on:submit={handleSubmit}>
		<input type="text" name="email" placeholder="Email" bind:value={email} />
		<input type="password" name="password" placeholder="Password" bind:value={password} />
		<button>Sign up</button>
	</form>
</div>