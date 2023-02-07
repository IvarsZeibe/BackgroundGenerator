<script lang="ts">
    import { onMount } from 'svelte';
    import { each } from 'svelte/internal';
    import backendService from '../services/backend-service';

    export let currentRoute;
    export let params;

    let users: any = {};

    onMount(async () => {
        users = await backendService.get_users();
        console.log(users);
        // .then(response => response.text())
        // .then(data => console.log(data));
    })

</script>

<div>
    <ul>
        {#each users as user}
            <li>{user.email} x {user.password}</li>
        {/each}
    </ul>
    <!-- {each }
    <form on:submit={handleSubmit}>
        <input type="text" name="email" placeholder="Email" bind:value={email} />
        <input type="password" name="password" placeholder="Password" bind:value={password} />
        <button>Login</button>
    </form> -->
</div>