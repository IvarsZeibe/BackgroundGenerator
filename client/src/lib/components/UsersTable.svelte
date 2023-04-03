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
	import validationHelper from '$lib/validation-helper';
	
	type DialogData = typeof editDialogData;
	
	let users: UserData[] = [];
	let editDialogData = {
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
		maxGenerators: {
			value: 10,
			isInvalid: false,
			errorMessage: ''
		},
	};
	let openRowIndex = 0;

	let sort: keyof UserData = 'id';
	let sortDirection: Lowercase<keyof typeof SortValue> = 'ascending';

	let isEditDialogOpen = false;
	let isClearGeneratorsDialogOpen = false;

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
		isEditDialogOpen = true;
		openRowIndex = index;
		setDialogData(users[index]);
		resetErrors();
	}
	function setDialogData(user: UserData) {
		editDialogData.id.value = user.id;
		editDialogData.email.value = user.email;
		editDialogData.password.value = '';
		editDialogData.isAdmin.value = user.isAdmin;
		editDialogData.maxGenerators.value = user.maxGenerators;
	} 
	function resetErrors() {
		Object.keys(editDialogData).forEach((k) => {
			let key = k as keyof DialogData;
			editDialogData[key].errorMessage = "";
			editDialogData[key].isInvalid = false;
		});
	}

	function validateId() {
		if (editDialogData.id.value <= 0) {
			editDialogData.id.isInvalid = true;
			editDialogData.id.errorMessage = "Must be atleast 1";
		} else {
			editDialogData.id.isInvalid = false;
			editDialogData.id.errorMessage = "";
		}
	}
	function validateMaxGenerators() {
		if (editDialogData.maxGenerators.value < 0) {
			editDialogData.maxGenerators.isInvalid = true;
			editDialogData.maxGenerators.errorMessage = "Must be atleast 0";
		} else {
			editDialogData.maxGenerators.isInvalid = false;
			editDialogData.maxGenerators.errorMessage = "";
		}
	}
	function validateEmail() {
		let error = validationHelper.validateEmail(editDialogData.email.value);
		if (error) {
			editDialogData.email.isInvalid = true;
			editDialogData.email.errorMessage = error;
		} else {
			editDialogData.email.isInvalid = false;
			editDialogData.email.errorMessage = "";
		}
	}
	function validatePassword() {
		let error = validationHelper.validatePassword(editDialogData.password.value);
		if (error && editDialogData.password.value.length != 0) {
			editDialogData.password.isInvalid = true;
			editDialogData.password.errorMessage = error;
		} else {
			editDialogData.password.isInvalid = false;
			editDialogData.password.errorMessage = "";
		}
	}
	async function handleSaveButton(e: CustomEvent<any>) {
		// prevents dialog from closing
		e.stopPropagation();

		if (isAnyDialogDataInvalid() || !(await trySave())) {
			return;
		}
		setUserDataFromDialog();
		isEditDialogOpen = false;
	}
	function setUserDataFromDialog() {
		users[openRowIndex].id = editDialogData.id.value;
		users[openRowIndex].email = editDialogData.email.value;
		users[openRowIndex].isAdmin = editDialogData.isAdmin.value;
		users[openRowIndex].maxGenerators = editDialogData.maxGenerators.value;
	}
	function isAnyDialogDataInvalid(): boolean {
		return Object.values(editDialogData)
			.some((value) => {
				return value.isInvalid
			});
	}
	// returns true if successful save
	async function trySave() {
		let data = await backendService.setUserData(
			users[openRowIndex].id, editDialogData.id.value, editDialogData.email.value,
			editDialogData.password.value, editDialogData.isAdmin.value, editDialogData.maxGenerators.value
		);
		Object.keys(data).forEach((k: string) => {
			let key = k as keyof DialogData;
			editDialogData[key].isInvalid = true;
			let key2 = k as keyof typeof data;
			editDialogData[key].errorMessage = data[key2];
		})
		return Object.keys(data).length == 0;
		
	}

	function openClearGeneratorsDialog(index: number) {
		isClearGeneratorsDialogOpen = true;
		openRowIndex = index;
	}
	function deleteAllUserGenerators() {
		backendService.deleteAllUserGenerators(users[openRowIndex].id);
		users[openRowIndex].generatorsSaved = 0;
		isClearGeneratorsDialogOpen = false;
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
			<Label>Is Admin</Label>
		</Cell>		
		<Cell numeric columnId="maxGenerators">
			<Label>Max Generators</Label>
			<IconButton class="material-icons">arrow_upward</IconButton>
		</Cell>
		<Cell numeric columnId="generatorsSaved">
			<Label>Generators Saved</Label>
			<IconButton class="material-icons">arrow_upward</IconButton>
		</Cell>
		<Cell numeric columnId="dateCreated">
			<Label>Created on</Label>
			<IconButton class="material-icons">arrow_upward</IconButton>
		</Cell>
		<Cell numeric columnId="lastAuthorized">
			<Label>Last authorized</Label>
			<IconButton class="material-icons">arrow_upward</IconButton>
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
			<Cell>{user.maxGenerators}</Cell>
			<Cell>{user.generatorsSaved}</Cell>
			<Cell>{user.dateCreated.toLocaleString("en-UK", {
				year: 'numeric', month: '2-digit', day: '2-digit',
				hour: "2-digit", minute: "2-digit", hour12: false
			})}</Cell>
			<Cell>{user.lastAuthorized.toLocaleString("en-UK", {
				year: 'numeric', month: '2-digit', day: '2-digit',
				hour: "2-digit", minute: "2-digit", hour12: false
			})}</Cell>
			<Cell><Button on:click={() => openClearGeneratorsDialog(i)}><ButtonLabel>Delete Generators</ButtonLabel></Button></Cell>
			<Cell><Button on:click={() => openEditDialog(i)}><ButtonLabel>Edit</ButtonLabel></Button></Cell>
		</Row>
		{/each}
	</Body>
</DataTable>

<Dialog
	bind:open={isEditDialogOpen}
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
				bind:value={editDialogData.id.value}
				bind:invalid={editDialogData.id.isInvalid}
				label="ID" type="number" input$min="1"
			>
				<HelperText validationMsg slot="helper">{editDialogData.id.errorMessage}</HelperText>
			</Textfield>
		</div>
		<div>
			<Textfield
				on:blur={validateEmail}
				bind:value={editDialogData.email.value}
				bind:invalid={editDialogData.email.isInvalid}
				type="email"
			>
				<svelte:fragment slot="label">
					<CommonIcon
						class="material-icons"
						style="font-size: 1em; line-height: normal; vertical-align: top;"
					>email</CommonIcon> Email
				</svelte:fragment>
				<HelperText validationMsg slot="helper">{editDialogData.email.errorMessage}</HelperText>
			</Textfield>
		</div>
		<div>
			<Textfield
				on:blur={validatePassword}
				bind:value={editDialogData.password.value}
				bind:invalid={editDialogData.password.isInvalid}
				label="Password" type="password"
			>
				<HelperText validationMsg slot="helper">{editDialogData.password.errorMessage}</HelperText>
			</Textfield>
		</div>
		<div>
			<FormField>
				<Switch bind:checked={editDialogData.isAdmin.value} />	
				<span slot="label">Is admin</span>
			</FormField>
		</div>
		<div>
			<Textfield
				on:blur={validateMaxGenerators}
				bind:value={editDialogData.maxGenerators.value}
				bind:invalid={editDialogData.maxGenerators.isInvalid}
				label="Max Generators" type="number"
			>
				<HelperText validationMsg slot="helper">{editDialogData.maxGenerators.errorMessage}</HelperText>
			</Textfield>
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

<Dialog
	bind:open={isClearGeneratorsDialogOpen}
>
	<Title id="mandatory-title">Clear All Generators?</Title>
	<Content id="mandatory-content">
		<div>
			Are you sure you want to clear all generators for this user?
		</div>
	</Content>
	<Actions>
		<Button on:click={deleteAllUserGenerators}>
			<ButtonLabel>Delete All Generators</ButtonLabel>
		</Button>
		<Button>
			<ButtonLabel>Cancel</ButtonLabel>
		</Button>
	</Actions>
</Dialog>
