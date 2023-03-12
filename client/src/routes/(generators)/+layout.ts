import { browser } from "$app/environment";
import backendService from "$lib/backend-service";
import type { LayoutLoad } from "../$types";

 
export const load = (async ({ url }) => {
	if (browser) {
		return {
			id: url.searchParams.get("id")
		}
	}
}) satisfies LayoutLoad;