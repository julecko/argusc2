import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import type { ProgramDetail, ProgramType, Capability } from '$lib/types';

export const load: PageServerLoad = async ({ params, cookies, fetch, locals }) => {
    if (!locals.user) redirect(302, '/login');

    const id    = parseInt(params.id, 10);
    const token = cookies.get('token');

    console.log('1. id:', id, 'token exists:', !!token);

    if (isNaN(id)) error(400, 'Invalid program ID');

    const headers = { Authorization: `Bearer ${token}` };

    console.log('2. fetching...');

    const [programRes, typesRes, capsRes] = await Promise.all([
        fetch(`/api/programs/${id}`, { headers }),
        fetch(`/api/program-types/all`,  { headers }),
        fetch(`/api/capabilities/all`,   { headers }),
    ]);

    console.log('3. statuses:', programRes.status, typesRes.status, capsRes.status);

    if (programRes.status === 404) error(404, 'Program not found');
    if (!programRes.ok) {
        const body = await programRes.text();
        console.log('4. programRes body:', body.substring(0, 500));
        error(500, 'Failed to load program');
    }

    console.log('5. parsing JSON...');  

    const program: ProgramDetail = await programRes.json();

    console.log('6. program:', JSON.stringify(program).substring(0, 200));

    console.log(typesRes, capsRes);

    const programTypes: ProgramType[] = typesRes.ok ? await typesRes.json() : [];
    const capabilities: Capability[]  = capsRes.ok  ? await capsRes.json()  : [];

    console.log('7. done. types:', programTypes.length, 'caps:', capabilities.length);

    return { program, programTypes, capabilities };
};

export const actions: Actions = {
    save: async ({ params, request, cookies, fetch, locals }) => {
        if (!locals.user) redirect(302, '/login');

        const token = cookies.get('token');
        const data = await request.formData();

        const body: Record<string, unknown> = {};

        const name = data.get('name') as string | null;
        const version = data.get('version') as string | null;
        const os = data.get('os') as string | null;
        const desc = data.get('description') as string | null;
        const allowed = data.get('allowed_downloads') as string | null;
        const ws_key = data.get('ws_key') as string | null;
        const type_id = data.get('type_id') as string | null;

        if (name !== null) body.name = name;
        if (version !== null) body.version = version;
        if (os !== null) body.os = os;
        if (desc !== null) body.description = desc;
        if (allowed !== null) body.allowed_downloads = Number(allowed);
        if (ws_key !== null) body.ws_key = ws_key;
        if (type_id !== null) body.type_id = Number(type_id);

        const capIds = data.getAll('capability_ids[]').map(Number).filter(Boolean);
        body.capability_ids = capIds;

        const res = await fetch(`/api/programs/${params.id}`, {
            method: 'PATCH',
            headers: {
                Authorization: `Bearer ${token}`,
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(body),
        });

        if (!res.ok) {
            const err = await res.json();
            return { success: false, error: err.error ?? 'Failed to save changes' };
        }

        const updated: ProgramDetail = await res.json();
        return { success: true, program: updated };
    },
};
