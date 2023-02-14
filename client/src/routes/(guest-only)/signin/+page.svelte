<script lang="ts">
	import backendService from '$lib/backend-service';

	let email = '';
	let password = '';

	async function handleSubmit(e: { preventDefault: () => void; }) {
		e.preventDefault();
		let { isOk, message } = await backendService.login(email, password);
		console.log("Is ok: " + isOk);
		console.log("Message: " + message);
	}
</script>

<h1>Login</h1>
<div>
	<form on:submit={handleSubmit}>
		<input type="text" name="email" placeholder="Email" bind:value={email} />
		<input type="password" name="password" placeholder="Password" bind:value={password} />
		<button>Login</button>
	</form>
</div>