import { redirect, error, type Handle } from '@sveltejs/kit';

const API_BASE = 'http://127.0.0.1:3000';

async function safeFetch(url: string, options?: RequestInit) {
    try {
        return await fetch(url, options);
    } catch {
        throw error(404, 'Backend unreachable');
    }
}

async function getSetupRequired(): Promise<boolean> {
    const res = await safeFetch(`${API_BASE}/api/auth/status`);

    if (!res.ok) {
        throw error(404, 'Backend unavailable');
    }

    const data = await res.json();
    return data.setup_required === true;
}

async function verifyToken(token: string): Promise<string | null> {
    const res = await safeFetch(`${API_BASE}/api/auth/verify`, {
        headers: { Cookie: `token=${token}` },
    });

    if (!res.ok) {
        return null;
    }

    const data = await res.json();
    return typeof data.username === 'string' ? data.username : null;
}

export const handle: Handle = async ({ event, resolve }) => {
    const path = event.url.pathname;

    if (path.startsWith('/api') || path.includes('.')) {
        return resolve(event);
    }

    const setupRequired = await getSetupRequired();

    if (setupRequired && path !== '/createadmin') {
        throw redirect(302, '/createadmin');
    }

    if (!setupRequired && path === '/createadmin') {
        throw redirect(302, '/login');
    }

    if (path.startsWith('/admin') || path === '/login') {
        const token = event.cookies.get('token');

        if (token) {
            const username = await verifyToken(token);

            if (username) {
                event.locals.user = { username };

                if (path === '/login') {
                    throw redirect(302, '/admin/dashboard');
                }
            } else {
                event.cookies.delete('token', { path: '/' });
            }
        }

        if (path.startsWith('/admin') && !event.locals.user) {
            throw redirect(302, '/login');
        }
    }

    return resolve(event);
};
