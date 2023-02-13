<script lang="ts">
    import { goto } from "$app/navigation";
    import { user } from "../../../stores";

    $: {
		if (!$user.getAuthorisedUserData().isAdmin) {
			goto("/");
		}
    }
</script>

{#if !$user.getAuthorisedUserData().isAdmin}
<p>loading..</p>
{:else}
<slot />
{/if}