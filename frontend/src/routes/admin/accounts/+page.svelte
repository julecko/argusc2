<script lang="ts">
    import PageHeader from '$lib/components/layout/PageHeader.svelte';
    import DataTable from '$lib/components/layout/DataTable.svelte';
    import type { Column, Row, BadgeVariant } from '$lib/types';

    const columns: Column[] = [
        { key: 'username', label: 'Username' },
        { key: 'email',    label: 'Email' },
        { key: 'role',     label: 'Role',       width: '120px' },
        { key: 'created',  label: 'Created' },
        { key: 'lastLogin', label: 'Last Login' }
    ];

    const rows: Row[] = [
        { username: 'admin',     email: 'admin@c2server.local',     role: 'admin',    created: 'Jan 15, 2024, 01:00 AM', lastLogin: 'Mar 21, 2026, 11:33 AM' },
        { username: 'operator1', email: 'operator1@c2server.local', role: 'operator', created: 'Feb 20, 2024, 01:00 AM', lastLogin: 'Mar 21, 2026, 10:33 AM' },
        { username: 'viewer1',   email: 'viewer1@c2server.local',   role: 'viewer',   created: 'Mar 10, 2024, 01:00 AM', lastLogin: 'Mar 21, 2026, 09:33 AM' }
    ];

    const statusColors: Record<string, BadgeVariant> = {
        admin:    'red',
        operator: 'blue',
        viewer:   'gray'
    };

    function handleAction(row: Row): void {
        console.log('Action for', row.username);
    }
</script>

<section class="page">
    <PageHeader title="Accounts" description="Manage user accounts and permissions" />
    <DataTable {columns} {rows} statusKey="role" {statusColors}>

        <svelte:fragment slot="cell" let:col let:row let:value>
            {#if col.key === 'username'}
                <strong class="username">{value}</strong>
            {:else}
                {value}
            {/if}
        </svelte:fragment>

        <svelte:fragment slot="actions" let:row>
            <button
                class="action-btn"
                on:click|stopPropagation={() => handleAction(row)}
                aria-label="More options for {row.username}"
            >
                ⋮
            </button>
        </svelte:fragment>

    </DataTable>
</section>

<style lang="scss">
    @use '$lib/styles/variables' as *;

    section.page {
        display: flex;
        flex-direction: column;
        gap: 35px;
        background: $bg-main;
    }

    .username {
        font-weight: 600;
        color: #d4d4dc;
    }

    .action-btn {
        background: none;
        border: none;
        color: #6b6b7e;
        cursor: pointer;
        font-size: 18px;
        padding: 2px 6px;
        border-radius: 4px;
        line-height: 1;
        transition: color 140ms ease, background 140ms ease;

        &:hover {
            color: #d4d4dc;
            background: rgba(255, 255, 255, 0.06);
        }
    }
</style>
