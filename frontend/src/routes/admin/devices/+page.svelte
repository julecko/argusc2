<script lang="ts">
	import PageHeader from '$lib/components/layout/PageHeader.svelte';
	import DataTable from '$components/layout/DataTable.svelte';
	import type { Column, Row, BadgeVariant, DeviceSummary } from '$lib/types';

	export let data: { devices: DeviceSummary[] };
	console.log(data);

	function mapDevice(d: DeviceSummary): Row {
		return {
			status: d.is_online === 'true' ? 'online' : 'offline',
			name: d.hostname,
			ip: d.ip_external,
			os: `${d.os} ${d.os_version}`,
			lastSeen: formatLastSeen(d.last_seen),
			uploads: 0
		};
	}

	function formatLastSeen(timestamp: number): string {
		const diff = Date.now() - timestamp * 1000;

		const minutes = Math.floor(diff / 60000);
		if (minutes < 1) return 'just now';
		if (minutes < 60) return `${minutes}m ago`;

		const hours = Math.floor(minutes / 60);
		if (hours < 24) return `${hours}h ago`;

		const days = Math.floor(hours / 24);
		return `${days}d ago`;
	}

	const columns: Column[] = [
		{ key: 'status', label: 'Status', width: '110px' },
		{ key: 'name', label: 'Name' },
		{ key: 'ip', label: 'IP Address' },
		{ key: 'os', label: 'Operating System' },
		{ key: 'lastSeen', label: 'Last Seen' },
		{ key: 'uploads', label: 'Uploads' }
	];

	const rows: Row[] = data.devices.map(mapDevice);

	const statusColors: Record<string, BadgeVariant> = {
		online: 'green',
		idle: 'amber',
		offline: 'gray'
	};

	function handleAction(row: Row): void {
		console.log('Action for', row.name);
	}

	function handleRow(row: Row): void {
		console.log('Row for', row.name);
	}
</script>

<section class="page">
	<PageHeader title="Devices" description="Manage all devices communicating with c2 server" />
	<DataTable {columns} {rows} statusKey="status" {statusColors} onRowClick={handleRow}>
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
				on:click|stopPropagation={() => handleAction(row)}
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
