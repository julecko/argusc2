<script lang="ts">
	import PageHeader from '$lib/components/layout/PageHeader.svelte';
	import DataTable from '$components/layout/DataTable.svelte';
    import type { Column, Row, BadgeVariant } from '$lib/types';

	const columns: Column[] = [
		{ key: 'status', label: 'Status', width: '110px' },
		{ key: 'name', label: 'Name' },
		{ key: 'ip', label: 'IP Address' },
		{ key: 'os', label: 'Operating System' },
		{ key: 'lastSeen', label: 'Last Seen' },
		{ key: 'uploads', label: 'Uploads' }
	];

    // Fake rows
	const rows: Row[] = [
		{
			status: 'online',
			name: 'DESKTOP-WIN10-01',
			ip: '192.168.1.105',
			os: 'Windows 10 Pro',
			lastSeen: '44m ago',
			uploads: 2
		},
		{
			status: 'online',
			name: 'UBUNTU-SERVER-02',
			ip: '192.168.1.112',
			os: 'Ubuntu 22.04 LTS',
			lastSeen: '49m ago',
			uploads: 1
		},
		{
			status: 'idle',
			name: 'MACBOOK-PRO-03',
			ip: '192.168.1.89',
			os: 'macOS Sonoma',
			lastSeen: '1h ago',
			uploads: 0
		},
		{
			status: 'offline',
			name: 'LAPTOP-CORP-04',
			ip: '10.0.0.45',
			os: 'Windows 11 Enterprise',
			lastSeen: '1d ago',
			uploads: 1
		}
	];

	const statusColors: Record<string, BadgeVariant> = {
		online: 'green',
		idle: 'amber',
		offline: 'gray'
	};

	function handleAction(row: Row): void {
		console.log('Action for', row.name);
	}
</script>

<section class="page">
	<PageHeader title="Devices" description="Manage all devices communicating with c2 server" />
	<DataTable {columns} {rows} statusKey="status" {statusColors}>
		<svelte:fragment slot="cell" let:col let:row let:value>
			{#if col.key === 'uploads'}
				<span class="uploads">
					<svg
						width="14"
						height="14"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" />
						<circle cx="12" cy="12" r="3" />
					</svg>
					{value}
					{value === 1 ? 'file' : 'files'}
				</span>
			{:else}
				{value}
			{/if}
		</svelte:fragment>

		<svelte:fragment slot="actions" let:row>
			<button
				class="action-btn"
				on:click={() => handleAction(row)}
				aria-label="More options for {row.name}"
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

	.uploads {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		color: #4a9eff;
		font-size: 13px;
	}

	.action-btn {
		background: none;
		border: none;
		color: #6b6b7e;
		cursor: pointer;
        text-align: center;
		font-size: 18px;
		padding: 2px 6px;
		border-radius: 4px;
		line-height: 1;
		transition:
			color 140ms ease,
			background 140ms ease;

		&:hover {
			color: #d4d4dc;
			background: rgba(255, 255, 255, 0.06);
		}
	}
</style>
