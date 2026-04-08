import { redirect } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import type { DeviceSummary } from '$lib/types';

export const load: PageServerLoad = async ({ cookies, fetch, locals }) => {
    if (!locals.user) redirect(302, '/login');

    const token = cookies.get('token');

    const res = await fetch('/api/devices/all', {
        headers: { Authorization: `Bearer ${token}` },
    });

    if (!res.ok) return { programs: [] };

    const devices: DeviceSummary[] = await res.json();
    return { devices };
};

/*export const actions: Actions = {
    delete: async ({ request, cookies, fetch, locals }) => {n
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
*/