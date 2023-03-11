import { browser } from "$app/environment";
import type { LayoutLoad } from "../$types";

 
export const load = (({ url }) => {
	let id: string | null = null;
	if (browser) {
		id = url.searchParams.get("id");
	}
	return { id };
}) satisfies LayoutLoad;