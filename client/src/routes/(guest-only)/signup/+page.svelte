<script lang="ts">
	import backendService from '$lib/backend-service';
    import validationHelper from '$lib/validation-helper';
    import Button, { Label as ButtonLabel } from '@smui/button';
    import { Icon } from '@smui/common';
    import Textfield from '@smui/textfield';
    import HelperText from '@smui/textfield/helper-text';

	let error = '';

	let email = '';
	let emailError = '';
	let isEmailInvalid = false;

	let password = '';
	let passwordError = '';
	let isPasswordInvalid = false;

	let passwordRepeat = '';
	let passwordRepeatError = '';
	let isPasswordRepeatInvalid = false;

	async function handleSubmit() {
		validateEmail();
		validatePassword();
		validatePasswordRepeat();
		if (isEmailInvalid || isPasswordInvalid || isPasswordRepeatInvalid) {
			return;
		}
		let { isOk, message } = await backendService.register(email, password);
		if (!isOk) {
			error = message;
		}
	}

	function validateEmail() {
		let error = validationHelper.validateEmail(email);
		if (error) {
			isEmailInvalid = true;
			emailError = error;
		} else {
			isEmailInvalid = false;
			emailError = "";
		}
	}
	function validatePassword() {
		let error = validationHelper.validatePassword(password);
		if (error) {
			isPasswordInvalid = true;
			passwordError = error;
		} else {
			isPasswordInvalid = false;
			passwordError = "";
		}
	}
	function validatePasswordRepeat() {
		if (password !== passwordRepeat) {
			isPasswordRepeatInvalid = true;
			passwordRepeatError = "Passwords don't match";
		} else {
			isPasswordRepeatInvalid = false;
			passwordRepeatError = "";
		}
	}
</script>

<svelte:head>
	<title>Sign up | Background generator</title>
</svelte:head>

<h1>Sign up</h1>

<div id="container">
	{#if error !== ''}
	<div id="error">{error}</div>
	{/if}
	<Textfield
		on:blur={validateEmail}
		bind:value={email}
		bind:invalid={isEmailInvalid}
		type="email"
	>
		<svelte:fragment slot="label">
			<Icon
				class="material-icons"
				style="font-size: 1em; line-height: normal; vertical-align: top;"
			>email</Icon> Email
		</svelte:fragment>
		<HelperText validationMsg slot="helper">{emailError}</HelperText>
	</Textfield>
	<Textfield
		on:blur={validatePassword}
		bind:value={password}
		bind:invalid={isPasswordInvalid}
		label="Password" type="password"
	>
		<HelperText validationMsg slot="helper">{passwordError}</HelperText>
	</Textfield>
	<Textfield
		on:blur={validatePasswordRepeat}
		bind:value={passwordRepeat}
		bind:invalid={isPasswordRepeatInvalid}
		label="Repeat password" type="password"
	>
		<HelperText validationMsg slot="helper">{passwordRepeatError}</HelperText>
	</Textfield>
</div>

<Button on:click={handleSubmit}>
	<ButtonLabel>Sign up</ButtonLabel>
</Button>
<a href="/signin">Already have an account? Sign in!</a>

<style>
#error {
	padding: 5px 10px;
	background: #fcd0cf;
	border-radius: 5px;
	border: #fea9a7 1px solid;
	width: 100%;
	color: #b61f1a;
	box-sizing: border-box;
}
#container {
	display: flex;
	flex-direction: column;
	width: 300px;
}
</style>