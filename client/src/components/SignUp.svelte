<script lang="ts">
    import { onMount } from 'svelte';
    import backendService from '../services/backend-service';

    export let currentRoute;
    export let params;

    let email = '';
    let password = '';

    function handleSubmit(e) {
        e.preventDefault();
        backendService.register(email, password).then(res => console.log(res));
    }

    onMount(async () => {
        backendService.test()
        .then(response => response.text())
        .then(data => console.log(data));
    })

</script>

<div>
    <form on:submit={handleSubmit}>
        <input type="text" name="email" placeholder="Email" bind:value={email} />
        <input type="password" name="password" placeholder="Password" bind:value={password} />
        <button>Login</button>
    </form>
</div>