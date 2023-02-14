<script lang="ts">
	import { goto } from "$app/navigation";
	import { user } from "../../stores";
	import { page } from "$app/stores";

	$: if ($user.isGuest()) {
		goto("/signin?next=" + encodeURIComponent($page.url.pathname.slice(0, -1)));
	}
</script>

{#if !$user.isAuthorised()}
<p>loading..</p>
{:else}
<slot />
{/if}