<script lang="ts">
	import PageHeader from '$lib/components/layout/PageHeader.svelte';
	import DataTable from '$components/layout/DataTable.svelte';
    import UploadProgramModal from '$lib/components/ui/modals/UploadProgramModal.svelte';
	import type { Column, Row, BadgeVariant } from '$lib/types';

	const columns: Column[] = [
		{ key: 'name', label: 'Name' },
		{ key: 'type', label: 'Type' },
		{ key: 'os', label: 'OS' },
		{ key: 'version', label: 'Version' },
		{ key: 'size', label: 'Size' },
		{ key: 'downloads', label: 'Downloads' },
		{ key: 'uploadedAt', label: 'Uploaded At' }
	];

	let rows: Row[] = [
		{
			name: 'payload_v2.exe',
			type: ['RAT', '#f87171'],
			os: 'Windows',
			version: 'v2.0',
			size: '1.95 MB',
			downloads: '5 / 10',
			uploadedAt: 'Mar 21, 2026, 12:02 PM'
		},
		{
			name: 'keylogger.dll',
			type: ['Keylogger', '#f5a623'],
			os: 'Windows',
			version: 'v1.0',
			size: '500.00 KB',
			downloads: '3 / 5',
			uploadedAt: 'Mar 20, 2026, 12:02 PM'
		},
		{
			name: 'screenshot_tool.py',
			type: ['Spyware', '#6b6b7e'],
			os: 'Linux',
			version: 'v1.0',
			size: '15.00 KB',
			downloads: '2 / ∞',
			uploadedAt: 'Mar 19, 2026, 12:02 PM'
		}
	];

	const typeColors: Record<string, BadgeVariant> = {
		RAT: 'red',
		Keylogger: 'amber',
		Spyware: 'gray',
		Ransomware: 'blue'
	};

	let openMenuId: string | null = null;

	function toggleMenu(name: string) {
		openMenuId = openMenuId === name ? null : name;
	}

	function closeMenu() {
		openMenuId = null;
	}

	function deleteProgram(row: Row) {
		rows = rows.filter((r) => r.name !== row.name);
		closeMenu();
	}

	function handleOutsideClick(e: MouseEvent) {
		if (!(e.target as HTMLElement).closest('.menu-wrap')) {
			closeMenu();
		}
	}

	let uploadOpen = false;
</script>

<svelte:window on:click={handleOutsideClick} />
<UploadProgramModal bind:open={uploadOpen} />

<section class="page">
	<PageHeader title="Programs" description="Manage and deploy programs to devices">
		<svelte:fragment slot="actions">
			<button class="upload-btn" on:click={() => (uploadOpen = true)}>
				<svg
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<polyline points="16 16 12 12 8 16" />
					<line x1="12" y1="12" x2="12" y2="21" />
					<path d="M20.39 18.39A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.3" />
				</svg>
				Upload Program
			</button>
		</svelte:fragment>
	</PageHeader>

	<DataTable {columns} {rows} statusKey="__none__">
		<svelte:fragment slot="cell" let:col let:row let:value>
			{#if col.key === 'name'}
				<span class="program-name">{value}</span>
			{:else if col.key === 'type'}
				{@const [label, color] = value as [string, string]}
				<span
					class="type-badge"
					style="color:{color}; background:{color}1a; border-color:{color}40;"
				>
					{label}
				</span>
			{:else if col.key === 'size'}
				<span class="size-text">{value}</span>
			{:else if col.key === 'downloads'}
				<span class="downloads-text">{value}</span>
			{:else if col.key === 'uploadedAt'}
				<span class="date-text">{value}</span>
			{:else}
				<span class="standard-text">{value}</span>
			{/if}
		</svelte:fragment>

		<svelte:fragment slot="actions" let:row>
			<div class="menu-wrap">
				<button
					class="action-btn"
					class:action-btn--active={openMenuId === String(row.name)}
					on:click|stopPropagation={() => toggleMenu(String(row.name))}
					aria-label="More options for {row.name}"
				>
					⋮
				</button>

				{#if openMenuId === String(row.name)}
					<div class="context-menu">
						<button
							class="context-item context-item--danger"
							on:click|stopPropagation={() => deleteProgram(row)}
						>
							<svg
								width="13"
								height="13"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<polyline points="3 6 5 6 21 6" />
								<path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6" />
								<path d="M10 11v6" /><path d="M14 11v6" />
								<path d="M9 6V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2" />
							</svg>
							Delete
						</button>
					</div>
				{/if}
			</div>
		</svelte:fragment>
	</DataTable>
</section>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	section.page {
		display: flex;
		flex-direction: column;
		gap: 10px;
		background: $bg-main;
	}

	.upload-btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		background: $accent;
		border: none;
		border-radius: $radius;
		color: white;
		font-size: 13px;
		font-weight: 600;
		padding: 9px 16px;
		cursor: pointer;
		transition:
			opacity $transition,
			transform $transition;

		&:hover {
			opacity: 0.88;
		}
		&:active {
			transform: scale(0.97);
		}
	}

	.program-name {
		font-family: 'JetBrains Mono', 'Fira Code', monospace;
		font-size: 13px;
		font-weight: 600;
		color: #d4d4dc;
	}

	.size-text {
		font-variant-numeric: tabular-nums;
		font-weight: 600;
		color: #d4d4dc;
	}

	.downloads-text {
		font-variant-numeric: tabular-nums;
		color: #9f9f9a;
	}

	.date-text, .standard-text {
		color: #9f9f9a;
		font-size: 12px;
	}

	.os-badge,
	.version-badge {
		font-family: 'JetBrains Mono', monospace;
		font-size: 10px;
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
		transition:
			color 140ms ease,
			background 140ms ease;

		&:hover,
		&--active {
			color: #d4d4dc;
			background: rgba(255, 255, 255, 0.06);
		}
	}

	.menu-wrap {
		position: relative;
	}

	.context-menu {
		position: absolute;
		top: auto;
		bottom: calc(100% + 4px);
		right: 0;
		background: #18181b;
		border: 1px solid #1f2933;
		border-radius: $radius;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
		overflow: hidden;
		min-width: 130px;
		z-index: 50;
		animation: pop 0.12s cubic-bezier(0.16, 1, 0.3, 1);
	}

	@keyframes pop {
		from {
			opacity: 0;
			transform: scale(0.95) translateY(4px);
		}
		to {
			opacity: 1;
			transform: scale(1) translateY(0);
		}
	}

	.type-badge {
		display: inline-flex;
		align-items: center;
		padding: 3px 10px;
		border-radius: 20px;
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.04em;
		border: 1px solid transparent;
	}

	.context-item {
		display: flex;
		align-items: center;
		gap: 8px;
		width: 100%;
		background: none;
		border: none;
		padding: 9px 14px;
		font-size: 13px;
		cursor: pointer;
		text-align: left;
		color: #d4d4dc;
		transition: background 140ms ease;

		&:hover {
			background: rgba(255, 255, 255, 0.05);
		}

		&--danger {
			color: #f87171;
			svg {
				color: #f87171;
			}
			&:hover {
				background: rgba(248, 113, 113, 0.08);
			}
		}
	}
</style>
