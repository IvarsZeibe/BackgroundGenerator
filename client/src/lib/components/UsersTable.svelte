<script lang="ts">
	import DataTable, {
	  Head,
	  Body,
	  Row,
	  Cell,
	  Label,
	  SortValue,
	} from '@smui/data-table';
	import IconButton from '@smui/icon-button';
  	import { Icon as CommonIcon } from '@smui/common';
	import Button, { Label as ButtonLabel } from '@smui/button';
	import Dialog, { Title, Content, Actions } from '@smui/dialog';
	import FormField from '@smui/form-field';
	import Textfield from '@smui/textfield';
	import Switch from '@smui/switch';
	import HelperText from '@smui/textfield/helper-text';
	import backendService, { type UserData } from '$lib/backend-service';
	import { onMount } from 'svelte';
	
	type DialogData = typeof dialogData;
	
	let users: UserData[] = [];
	let dialogData = {
		id: {
			value: 1,
			isInvalid: false,
			errorMessage: ''
		},
		email: {
			value: '',
			isInvalid: false,
			errorMessage: ''
		},
		password: {
			value: '',
			isInvalid: false,
			errorMessage: ''
		},
		isAdmin: {
			value: false,
			isInvalid: false,
			errorMessage: ''
		},
	};
	let openRowIndex = 0;

	let sort: keyof UserData = 'id';
	let sortDirection: Lowercase<keyof typeof SortValue> = 'ascending';

	let isDialogOpen = false;
	let isExtraInfoShown = false;

	onMount(async () => {
		users = await backendService.getUsers();
	});
   
	function handleSort() {
	  	users.sort((a, b) => {
			const [aVal, bVal] = [a[sort], b[sort]][
				sortDirection === 'ascending' ? 'slice' : 'reverse'
			]();
			if (typeof aVal === 'string' && typeof bVal === 'string') {
				return aVal.localeCompare(bVal);
			}
			return Number(aVal) - Number(bVal);
		});
	  	users = users;
	}

	function openEditDialog(index: number) {
		isDialogOpen = true;
		openRowIndex = index;
		setDialogData(users[index]);
		resetErrors();
	}
	function setDialogData(user: UserData) {
		dialogData.id.value = user.id;
		dialogData.email.value = user.email;
		dialogData.password.value = '';
		dialogData.isAdmin.value = user.isAdmin;
	} 
	function resetErrors() {
		Object.keys(dialogData).forEach((k) => {
			let key = k as keyof DialogData;
			dialogData[key].errorMessage = "";
			dialogData[key].isInvalid = false;
		});
	}

	function validateId() {
		if (dialogData.id.value <= 0) {
			dialogData.id.isInvalid = true;
			dialogData.id.errorMessage = "Must be atleast 1";
		} else {
			dialogData.id.isInvalid = false;
			dialogData.id.errorMessage = "";
		}
	}
	function validateEmail() {
		let validEmailPattern = /^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,253}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,253}[a-zA-Z0-9])?)*$/;
		if (validEmailPattern.test(dialogData.email.value)) {
			dialogData.email.isInvalid = false;
			dialogData.email.errorMessage = "";
		} else {
			dialogData.email.isInvalid = true;
			dialogData.email.errorMessage = "Must be a valid email";
		}
	}
	function validatePassword() {
		if (dialogData.password.value.length < 8 && dialogData.password.value.length != 0) {
			dialogData.password.isInvalid = true;
			dialogData.password.errorMessage = "must be atleast 8 characters";
		} else {
			dialogData.password.isInvalid = false;
			dialogData.password.errorMessage = "";
		}
	}
	async function handleSaveButton(e: CustomEvent<any>) {
		// prevents dialog from closing
		e.stopPropagation();

		if (isAnyDialogDataInvalid() || !(await trySave())) {
			return;
		}
		setUserDataFromDialog();
		isDialogOpen = false;
	}
	function setUserDataFromDialog() {
		users[openRowIndex].id = dialogData.id.value;
		users[openRowIndex].email = dialogData.email.value;
		users[openRowIndex].isAdmin = dialogData.isAdmin.value;
	}
	function isAnyDialogDataInvalid(): boolean {
		return Object.values(dialogData)
			.some((value) => {
				return value.isInvalid
			});
	}
	// returns true if successful save
	async function trySave() {
		let data = await backendService.setUserData(
			users[openRowIndex].id, dialogData.id.value, dialogData.email.value,
			dialogData.password.value, dialogData.isAdmin.value
		);
		Object.keys(data).forEach((k: string) => {
			let key = k as keyof DialogData;
			dialogData[key].isInvalid = true;
			let key2 = k as keyof typeof data;
			dialogData[key].errorMessage = data[key2];
		})
		return Object.keys(data).length == 0;
		
	}
</script>

<Button on:click={() => {isExtraInfoShown = !isExtraInfoShown}}>Show extra info</Button>

<DataTable
	sortable
	bind:sort
	bind:sortDirection
	on:SMUIDataTable:sorted={handleSort}
	table$aria-label="User list"
	style="width: 100%;"
>
	<Head>
		<Row>
		{#if isExtraInfoShown}
		<Cell numeric columnId="id">
			<IconButton class="material-icons">arrow_upward</IconButton>
			<Label>ID</Label>
		</Cell>
		{/if}
		<Cell columnId="email" style="width: 100%;">
			<Label>Email</Label>
			<IconButton class="material-icons">arrow_upward</IconButton>
		</Cell>
		<Cell sortable={false}>
			<Label>IsAdmin</Label>
		</Cell>
		</Row>
	</Head>
	<Body>
		{#each users as user, i (user.id)}
		<Row>
			{#if isExtraInfoShown}
			<Cell numeric>{user.id}</Cell>
			{/if}
			<Cell>{user.email}</Cell>
			<Cell>{user.isAdmin}</Cell>
			<Cell><Button on:click={() => openEditDialog(i)}><ButtonLabel>Edit</ButtonLabel></Button></Cell>
		</Row>
		{/each}
	</Body>
</DataTable>

<Dialog
	bind:open={isDialogOpen}
	scrimClickAction=""
	escapeKeyAction=""
	aria-labelledby="mandatory-title"
	aria-describedby="mandatory-content"
>
	<Title id="mandatory-title">Edit user</Title>
	<Content id="mandatory-content">
		<div>
			<Textfield
				on:blur={validateId}
				bind:value={dialogData.id.value}
				bind:invalid={dialogData.id.isInvalid}
				label="ID" type="number" input$min="1"
			>
				<HelperText validationMsg slot="helper">{dialogData.id.errorMessage}</HelperText>
			</Textfield>
		</div>
		<div>
			<Textfield
				on:blur={validateEmail}
				bind:value={dialogData.email.value}
				bind:invalid={dialogData.email.isInvalid}
				type="email"
			>
				<svelte:fragment slot="label">
					<CommonIcon
						class="material-icons"
						style="font-size: 1em; line-height: normal; vertical-align: top;"
					>email</CommonIcon> Email
				</svelte:fragment>
				<HelperText validationMsg slot="helper">{dialogData.email.errorMessage}</HelperText>
			</Textfield>
		</div>
		<div>
			<Textfield
				on:blur={validatePassword}
				bind:value={dialogData.password.value}
				bind:invalid={dialogData.password.isInvalid}
				label="Password" type="password"
			>
				<HelperText validationMsg slot="helper">{dialogData.password.errorMessage}</HelperText>
			</Textfield>
		</div>
		<div>
			<FormField>
				<Switch bind:checked={dialogData.isAdmin.value} />	
				<span slot="label">Is admin</span>
			</FormField>
		</div>
	</Content>
	<Actions>
		<Button on:click={handleSaveButton}>
			<ButtonLabel>Save</ButtonLabel>
		</Button>
		<Button>
			<ButtonLabel>Cancel</ButtonLabel>
		</Button>
	</Actions>
</Dialog>
