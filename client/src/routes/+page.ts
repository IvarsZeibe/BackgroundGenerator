import backendService from '$lib/backend-service';
import type { PageLoad } from './$types';
 
export const load = (async () => {
	return {
		generators: await backendService.getGenerators()
	};
}) satisfies PageLoad;