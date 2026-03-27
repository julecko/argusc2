import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import type { Account } from '$lib/types';

export const load: PageServerLoad = async ({ params, cookies, fetch, locals }) => {
    if (!locals.user) redirect(302, '/login');

    const token = cookies.get('token');

    const headers = { Authorization: `Bearer ${token}` };

    const accountsRes = await fetch(`/api/accounts/all`, { headers });

    if (accountsRes.status === 404) error(404, 'Accounts not found');
    if (!accountsRes.ok) {
        error(500, 'Failed to load program');
    }

    const accounts: Account[] = await accountsRes.json();

    return { accounts };
};
