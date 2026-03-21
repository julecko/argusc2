import { redirect, type Handle } from '@sveltejs/kit';

const API_BASE = 'http://127.0.0.1:3000';

async function getSetupRequired(): Promise<boolean> {
    try {
        const res = await fetch(`${API_BASE}/api/auth/status`);
        if (!res.ok) return false;
        const data = await res.json();
        return data.setup_required === true;
    } catch {
        return false;
    }
}

async function verifyToken(token: string): Promise<string | null> {
    try {
        const res = await fetch(`${API_BASE}/api/auth/verify`, {
            headers: { Authorization: `Bearer ${token}` },
        });
        if (!res.ok) return null;
        const data = await res.json();
        return typeof data.username === 'string' ? data.username : null;
    } catch {
        return null;
    }
}

export const handle: Handle = async ({ event, resolve }) => {
    const path = event.url.pathname;

    if (path.startsWith('/api') || path.includes('.')) {
        return resolve(event);
    }

    const setupRequired = await getSetupRequired();

    if (setupRequired && path !== '/createadmin') {
        redirect(302, '/createadmin');
    }

    if (!setupRequired && path === '/createadmin') {
        redirect(302, '/login');
    }

    if (path.startsWith('/admin') || path === '/login') {
        const token = event.cookies.get('token');

        if (token) {
            const username = await verifyToken(token);

            if (username) {
                event.locals.user = { username };

                if (path === '/login') {
                    redirect(302, '/admin/dashboard');
                }
            } else {
                event.cookies.delete('token', { path: '/' });
            }
        }

        if (path.startsWith('/admin') && !event.locals.user) {
            redirect(302, '/login');
        }
    }

    return resolve(event);
};
