<script lang="ts">
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import PageHeader from '$lib/components/layout/PageHeader.svelte';
	import DataTable from '$components/layout/DataTable.svelte';
	import UploadProgramModal from '$lib/components/ui/modals/UploadProgramModal.svelte';
	import type { Column, Row, Program } from '$lib/types';
	import { downloadsDisplay, formatBytes, formatDate } from '$lib/utils/programs';

	export let data: { programs: Program[] };
	export let form: { success?: boolean; error?: string } | null = null;

	const columns: Column[] = [
		{ key: 'name', label: 'Name' },
		{ key: 'type_name', label: 'Type' },
		{ key: 'os', label: 'OS' },
		{ key: 'version', label: 'Version' },
		{ key: 'filesize', label: 'Size' },
		{ key: 'downloads', label: 'Downloads' },
		{ key: 'created_at', label: 'Uploaded At' }
	];

	$: rows = data.programs.map((p) => ({
		...p,
		downloads:
			downloadsDisplay(p.downloads, p.allowed_downloads),
		filesize: formatBytes(p.filesize)
	})) as Row[];

	let openMenuId: number | null = null;

	function toggleMenu(id: number) {
		openMenuId = openMenuId === id ? null : id;
	}

	function closeMenu() {
		openMenuId = null;
	}

	function handleOutsideClick(e: MouseEvent) {
		if (!(e.target as HTMLElement).closest('.menu-wrap')) closeMenu();
	}

	let uploadOpen = false;

	function handleUploadSuccess(program: Program) {
		data.programs = [program, ...data.programs];
	}
</script>

<svelte:window on:click={handleOutsideClick} />

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

	{#if form?.error}
		<div class="error-banner">{form.error}</div>
	{/if}

	<DataTable
		{columns}
		{rows}
		statusKey="__none__"
		onRowClick={(row) => goto(`/admin/programs/${(row as unknown as Program).id}`)}
	>
		<svelte:fragment slot="cell" let:col let:row let:value>
			{#if col.key === 'name'}
				<span class="program-name">{value}</span>
			{:else if col.key === 'type_name'}
				{@const program = data.programs.find((p) => p.name === String((row as Row).name))}
				{@const color = program?.type_color ?? '#6b6b7e'}
				<span
					class="type-badge"
					style="color:{color}; background:{color}1a; border-color:{color}40;"
				>
					{value ?? '—'}
				</span>
			{:else if col.key === 'os'}
				<span class="dt-badge dt-badge--gray os-badge">{value}</span>
			{:else if col.key === 'version'}
				<span class="dt-badge dt-badge--gray version-badge">{value}</span>
			{:else if col.key === 'filesize'}
				<span class="size-text">{value}</span>
			{:else if col.key === 'downloads'}
				<span class="downloads-text">{value}</span>
			{:else if col.key === 'created_at'}
				<span class="date-text">{formatDate(String(value))}</span>
			{:else}
				{value}
			{/if}
		</svelte:fragment>

		<svelte:fragment slot="actions" let:row>
			{@const program = row as unknown as Program}
			<div class="menu-wrap">
				<button
					class="action-btn"
					class:action-btn--active={openMenuId === program.id}
					on:click|stopPropagation={() => toggleMenu(program.id)}
					aria-label="More options">⋮</button
				>

				{#if openMenuId === program.id}
					<div class="context-menu">
						<button
							class="context-item"
							on:click|stopPropagation={() => {
								closeMenu();
								goto(`/admin/programs/${program.id}`);
							}}
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
								<path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
								<path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
							</svg>
							Edit
						</button>

						<form
							method="POST"
							action="?/delete"
							use:enhance={() => {
								closeMenu();
								return async ({ update }) => update();
							}}
						>
							<input type="hidden" name="id" value={program.id} />
							<button type="submit" class="context-item context-item--danger">
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
						</form>
					</div>
				{/if}
			</div>
		</svelte:fragment>
	</DataTable>

	<UploadProgramModal bind:open={uploadOpen} onSuccess={handleUploadSuccess} />
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

	.error-banner {
		background: rgba(#f87171, 0.1);
		border: 1px solid rgba(#f87171, 0.3);
		border-radius: $radius;
		padding: 10px 14px;
		font-size: 13px;
		color: #f87171;
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
		color: #9f9f9a;
		font-variant-numeric: tabular-nums;
	}
	.date-text {
		color: #9f9f9a;
		font-size: 12px;
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

	.dt-badge {
		display: inline-flex;
		align-items: center;
		padding: 3px 10px;
		border-radius: 20px;
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.04em;
		border: 1px solid transparent;
		&--gray {
			background: #1e1e26;
			color: #6b6b7e;
			border-color: #2e2e3e;
		}
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
