<script lang="ts">
    import { goto } from '$app/navigation';
    import backendService from '$lib/backend-service';
    import { onMount } from 'svelte';

    onMount(() => {
        backendService.logout();
        goto("/");
    });
</script>