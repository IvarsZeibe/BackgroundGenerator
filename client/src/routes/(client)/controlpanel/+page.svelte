<script lang="ts">
    import backendService from '$lib/backend-service';
    import { onMount } from 'svelte';
    
    let users: any[] = [];
    onMount(async () => {
        users = await backendService.get_users();
    })
</script>

<div>
    <ul>
        {#each users as user}
            <li>{user.email} x {user.password} x {user.is_admin}</li>
        {/each}
    </ul>
</div>