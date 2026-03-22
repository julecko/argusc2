import { redirect } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import type { Program } from '$lib/types';

export const load: PageServerLoad = async ({ cookies, fetch, locals }) => {
	if (!locals.user) redirect(302, '/login');

	const token = cookies.get('token');

	const res = await fetch('/api/programs', {
		headers: { Authorization: `Bearer ${token}` },
	});

	if (!res.ok) return { programs: [] };

	const programs: Program[] = await res.json();
	return { programs };
};

export const actions: Actions = {
	delete: async ({ request, cookies, fetch, locals }) => {
		if (!locals.user) redirect(302, '/login');

		const token = cookies.get('token');
		const data  = await request.formData();
		const id    = data.get('id');

		const res = await fetch(`/api/programs/${id}`, {
			method:  'DELETE',
			headers: { Authorization: `Bearer ${token}` },
		});

		if (!res.ok) {
			const body = await res.json();
			return { success: false, error: body.error ?? 'Delete failed' };
		}

		return { success: true };
	},
};
