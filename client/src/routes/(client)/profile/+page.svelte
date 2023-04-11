<script lang="ts">
    import backendService from '$lib/backend-service';
    import Button from '@smui/button/src/Button.svelte';
	import Select, { Option } from '@smui/select';
    import Textfield from '@smui/textfield';
    import HelperText from '@smui/textfield/helper-text';
    import { ThemeMode, themeMode, user } from '../../../stores';
	import ValidationHelper from '$lib/validation-helper';
    import Dialog, { Actions, Content, Title } from '@smui/dialog';
    import { goto } from '$app/navigation';

	let options = [ThemeMode.UseDeviceTheme, ThemeMode.Light, ThemeMode.Dark];
	let value: ThemeMode | "loading" = "loading";

	$: if (value == "loading" && $themeMode != null) {
		value = $themeMode;
	}
	let email = $user.getAuthorisedUserData().email;
	let isEmailInvalid = false;
	let emailError = "";

	let oldPassword = "";
	let isOldPasswordInvalid = false;
	let oldPasswordError = "";

	let newPassword = "";
	let comfirmNewPassword = "";
	let isNewPasswordInvalid = false;
	let newPasswordError = "";

	let isEditingPassword = false;
	let isDeleteAccountDialogOpen = false;

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
	
	async function changeEmail() {
		isEmailInvalid = false;
		emailError = "";
		
		let error = ValidationHelper.validateEmail(email)
		if (error) {
			isEmailInvalid = true;
			emailError = error;
			return;
		}
		let response = await backendService.changeEmail(email);
		if (response.ok) {
			isEmailInvalid = false;
			emailError = "";
		} else {
			isEmailInvalid = true;
			emailError = await response.text();
		}
	}

	async function changePassword() {
		isNewPasswordInvalid = false;
		newPasswordError = "";
		isOldPasswordInvalid = false;
		oldPasswordError = "";

		if (newPassword != comfirmNewPassword) {
			isNewPasswordInvalid = true;
			newPasswordError = "Passwords do not match";
			return;
		}
		let error = ValidationHelper.validatePassword(newPassword)
		if (error) {
			isNewPasswordInvalid = true;
			newPasswordError = error;
			return;
		}
		let response = await backendService.changePassword(oldPassword, newPassword);
		if (response.ok) {
			stopEditingPassword();
		} else {
			isOldPasswordInvalid = true;
			oldPasswordError = await response.text();
		}
	}
	function stopEditingPassword() {
		isEditingPassword = false;
		oldPassword = "";
		newPassword = "";
		comfirmNewPassword = "";
	}
	async function handleDeleteAccount() {
		if ((await backendService.deleteMyAccount()).ok) {
			await backendService.logout();
			goto("/");
		}
	}
</script>

<svelte:head>
	<title>Profile | Background generator</title>
</svelte:head>

<h1>Your profile</h1>

<div class="container">
	<h2>Preferred theme</h2>
	<div class="input-group">
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
	</div>

	<h2>Email</h2>
	<div class="input-group">
		<Textfield
			bind:value={email}
			bind:invalid={isEmailInvalid}
			label="Email"
		>
			<HelperText validationMsg slot="helper">{emailError}</HelperText>
		</Textfield>
		<Button on:click={() => {changeEmail()
		}}>Save</Button>			
	</div>

	<div>
		<h2>Password</h2>
		{#if !isEditingPassword}
		<Button on:click={() => {
			isEditingPassword = true;
		}}>Change</Button>
		{:else}
		<Textfield
			bind:value={oldPassword}
			label="Old password"
			bind:invalid={isOldPasswordInvalid}
			type="password"
		>
			<HelperText validationMsg slot="helper">{oldPasswordError}</HelperText>
		</Textfield><br>
		<Textfield
			bind:value={newPassword}
			label="New password"
			bind:invalid={isNewPasswordInvalid}
			type="password"
		>
			<HelperText validationMsg slot="helper">{newPasswordError}</HelperText>
		</Textfield><br>
		<Textfield
			bind:value={comfirmNewPassword}
			label="Comfirm new password"
			bind:invalid={isNewPasswordInvalid}
			type="password"
		>
			<HelperText validationMsg slot="helper">{newPasswordError}</HelperText>
		</Textfield><br>
		<Button on:click={() => {changePassword()
		}}>Save</Button>
		<Button on:click={() => {stopEditingPassword()
		}}>Cancel</Button>
		{/if}
	</div>
	<div>
		<h2>Account</h2>
		<Button on:click={() => {
			isDeleteAccountDialogOpen = true;
		}}>Delete account</Button>
	</div>	
</div>

<Dialog
	bind:open={isDeleteAccountDialogOpen}
>
	<Title>Delete account?</Title>
	<Content>
		<div>
			Are you sure you want to delete your account? This action cannot be undone.
		</div>
	</Content>
	<Actions>
		<Button on:click={() => handleDeleteAccount()}>
			Delete account
		</Button>
		<Button>
			Cancel
		</Button>
	</Actions>
</Dialog>

<style>
.container {
	display: flex;
	flex-direction: column;
}
.input-group {
	display: flex;
	flex-direction: row;
	align-items: center;
}
</style>