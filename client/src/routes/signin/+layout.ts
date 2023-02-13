import { browser } from "$app/environment";
import type { LayoutLoad } from "./$types";

export const load = (({ url }) => {
    let next = "";
    if (browser) {
        next += url.searchParams.get('next') || "";
    }
    console.log(next);
    return { next };
}) satisfies LayoutLoad;