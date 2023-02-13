import { browser } from "$app/environment";
import type { LayoutLoad } from "./$types";

export const load = (({ url }) => {
    let next = "";
    if (browser) {
        next += url.searchParams.get('next') || "/";
    }
    return { next };
}) satisfies LayoutLoad;