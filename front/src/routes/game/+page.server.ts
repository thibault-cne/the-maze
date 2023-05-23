import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
	const cookie = cookies.get('the_maze');

	return {
		cookie
	};
}) satisfies PageServerLoad;
