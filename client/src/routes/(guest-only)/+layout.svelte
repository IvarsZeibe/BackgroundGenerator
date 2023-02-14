<script lang="ts">
	import { browser } from "$app/environment";
	import { goto } from "$app/navigation";
	import { user } from "../../stores";
	import type { LayoutData } from "./$types";

	export let data: LayoutData;
	$: if (browser && $user.isAuthorised()) {
		goto(data.next);
	}
</script>

{#if $user.isGuest()}
<slot />
{:else}
<p>loading..</p>
{/if}