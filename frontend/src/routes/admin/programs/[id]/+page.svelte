<script lang="ts">
	import { enhance } from '$app/forms';
	import { fade } from 'svelte/transition';
	import { goto } from '$app/navigation';
	import PageHeader from '$lib/components/layout/PageHeader.svelte';
	import CapabilityGrid from '$components/ui/form/CapabilityGrid.svelte';
	import DownloadCommands from '$components/ui/DownloadCommands.svelte';
	import { downloadsDisplay, formatBytes, formatDate } from '$lib/utils/programs';
	import type { ProgramDetail, ProgramType, Capability, ProgramOS } from '$lib/types';

	export let data: {
		program: ProgramDetail;
		programTypes: ProgramType[];
		capabilities: Capability[];
	};
	export let form: { success?: boolean; error?: string; program?: ProgramDetail } | null = null;

	let editing = false;
	let saved = false;
	let draft = { ...data.program };
	let draftCaps: Set<number> = new Set(data.program.capabilities.map((c) => c.id));

	let savedTimer: ReturnType<typeof setTimeout>;

	function startEdit() {
		draft = { ...data.program };
		draftCaps = new Set(data.program.capabilities.map((c) => c.id));
		editing = true;
		saved = false;
	}

	function cancelEdit() {
		draft = { ...data.program };
		draftCaps = new Set(data.program.capabilities.map((c) => c.id));
		editing = false;
	}

	$: if (form?.success && form.program) {
		data.program = { ...form.program };
		draft = { ...form.program };
		draftCaps = new Set(form.program.capabilities.map((c) => c.id));
		editing = false;
		saved = true;
		clearTimeout(savedTimer);
		savedTimer = setTimeout(() => {
			saved = false;
		}, 3000);
	}

	const osList = ['windows', 'linux', 'android', 'mac'];

	const osLabels: Record<ProgramOS, string> = {
		windows: 'Windows',
		linux: 'Linux',
		android: 'Android',
		mac: 'macOS'
	};

	$: typeColor = data.program.type_color ?? '#6b6b7e';
	$: typeName = data.program.type_name ?? 'Unknown';
	$: downloads = downloadsDisplay(data.program.downloads, data.program.allowed_downloads);
	$: selectedType = data.programTypes.find((t) => t.id === draft.type_id);
</script>

<section class="page">
	<PageHeader title={data.program.name} description="Program details and configuration">
		<svelte:fragment slot="actions">
			{#if editing}
				<button class="btn btn--ghost" on:click={cancelEdit}>Cancel</button>
				<button class="btn btn--primary" form="edit-form" type="submit">Save Changes</button>
			{:else}
				<button class="btn btn--ghost" on:click={() => goto('/admin/programs')}>← Back</button>
				<button class="btn btn--primary" on:click={startEdit}>Edit Program</button>
			{/if}
		</svelte:fragment>
	</PageHeader>

	{#if saved}
		<div class="save-banner" transition:fade={{ duration: 400 }}>
			<svg
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2.5"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<polyline points="20 6 9 17 4 12" />
			</svg>
			Changes saved successfully.
		</div>
	{/if}

	{#if form?.error}
		<div class="error-banner">{form.error}</div>
	{/if}

	<form id="edit-form" method="POST" action="?/save" use:enhance>
		{#if editing}
			{#each [...draftCaps] as capId}
				<input type="hidden" name="capability_ids[]" value={capId} />
			{/each}
		{/if}

		<div class="grid">
			<!-- ── Left column ── -->
			<div class="col">
				<!-- Identity -->
				<div class="card">
					<div class="card-header"><span class="card-title">Identity</span></div>
					<div class="card-body">
						<div class="field">
							<span class="field-label">Name</span>
							{#if editing}
								<input name="name" class="field-input" bind:value={draft.name} />
							{:else}
								<span class="field-value mono">{data.program.name}</span>
							{/if}
						</div>

						<div class="field">
							<span class="field-label">Original filename</span>
							<span class="field-value mono muted">{data.program.original_name}</span>
						</div>

						<div class="field">
							<span class="field-label">Version</span>
							{#if editing}
								<input name="version" class="field-input" bind:value={draft.version} />
							{:else}
								<span class="field-value">{data.program.version}</span>
							{/if}
						</div>

						<div class="field">
							<span class="field-label">Type</span>
							{#if editing}
								<div class="field-right">
									<div class="select-wrap-sm">
										<select name="type_id" class="field-select" bind:value={draft.type_id}>
											{#each data.programTypes as t}
												<option value={t.id}>{t.name}</option>
											{/each}
										</select>
										<svg
											class="chevron"
											width="10"
											height="10"
											viewBox="0 0 24 24"
											fill="none"
											stroke="currentColor"
											stroke-width="2.5"
											stroke-linecap="round"
											stroke-linejoin="round"
										>
											<polyline points="6 9 12 15 18 9" />
										</svg>
									</div>
									{#if selectedType}
										<span
											class="type-preview"
											style="color:{selectedType.color}; background:{selectedType.color}1a; border-color:{selectedType.color}40;"
										>
											{selectedType.name}
										</span>
									{/if}
								</div>
							{:else}
								<span
									class="type-badge"
									style="color:{typeColor}; background:{typeColor}1a; border-color:{typeColor}40;"
								>
									{typeName}
								</span>
							{/if}
						</div>

						<div class="field">
							<span class="field-label">Operating System</span>
							{#if editing}
								<div class="select-wrap-sm">
									<select name="os" class="field-select" bind:value={draft.os}>
										{#each osList as o}
											<option value={o}>{osLabels[o as ProgramOS]}</option>
										{/each}
									</select>
									<svg
										class="chevron"
										width="10"
										height="10"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2.5"
										stroke-linecap="round"
										stroke-linejoin="round"
									>
										<polyline points="6 9 12 15 18 9" />
									</svg>
								</div>
							{:else}
								<span class="field-value">{osLabels[data.program.os]}</span>
							{/if}
						</div>
					</div>
				</div>

				<!-- Capabilities — shared CapabilityGrid component -->
				<div class="card">
					<div class="card-header"><span class="card-title">Capabilities</span></div>
					<div class="card-body">
						{#if editing}
							<CapabilityGrid options={data.capabilities} bind:selected={draftCaps} />
						{:else if data.program.capabilities.length > 0}
							<div class="cap-tags">
								{#each data.program.capabilities as cap}
									<span class="cap-tag">{cap.name}</span>
								{/each}
							</div>
						{:else}
							<p class="muted-text">No capabilities assigned.</p>
						{/if}
					</div>
				</div>

				<!-- Description -->
				<div class="card">
					<div class="card-header"><span class="card-title">Description</span></div>
					<div class="card-body">
						{#if editing}
							<textarea
								name="description"
								class="field-textarea"
								rows="5"
								placeholder="Enter program description…"
								bind:value={draft.description}
							></textarea>
						{:else}
							<p class="description-text">
								{data.program.description || 'No description provided.'}
							</p>
						{/if}
					</div>
				</div>

				<!-- Download Commands -->
				<div class="card">
					<div class="card-header"><span class="card-title">Download Commands</span></div>
					<div class="card-body">
						<DownloadCommands filename={data.program.original_name} file_hash={data.program.file_hash} />
					</div>
				</div>
			</div>

			<!-- ── Right column ── -->
			<div class="col">
				<!-- Statistics -->
				<div class="card">
					<div class="card-header"><span class="card-title">Statistics</span></div>
					<div class="card-body">
						<div class="field">
							<span class="field-label">Downloads</span>
							<span class="field-value accent">{downloads}</span>
						</div>

						<div class="field">
							<span class="field-label">Allowed Downloads</span>
							{#if editing}
								<div class="field-right">
									<input
										name="allowed_downloads"
										class="field-input"
										type="number"
										min="-1"
										bind:value={draft.allowed_downloads}
										on:input={() => {
											if (draft.allowed_downloads < -1) draft.allowed_downloads = -1;
										}}
									/>
									<span class="field-hint">-1 = unlimited · 0 = forbidden</span>
								</div>
							{:else}
								<span class="field-value" class:forbidden={data.program.allowed_downloads === 0}>
									{#if data.program.allowed_downloads === -1}∞ unlimited
									{:else if data.program.allowed_downloads === 0}⛔ forbidden
									{:else}{data.program.allowed_downloads}{/if}
								</span>
							{/if}
						</div>

						<div class="field">
							<span class="field-label">File Size</span>
							<span class="field-value">{formatBytes(data.program.filesize)}</span>
						</div>

						<div class="field">
							<span class="field-label">Uploaded by</span>
							<span class="field-value">{data.program.uploader_name ?? '—'}</span>
						</div>

						<div class="field">
							<span class="field-label">Created</span>
							<span class="field-value muted">{formatDate(data.program.created_at)}</span>
						</div>

						<div class="field">
							<span class="field-label">Last updated</span>
							<span class="field-value muted">{formatDate(data.program.updated_at)}</span>
						</div>
					</div>
				</div>

				<!-- Integrity -->
				<div class="card">
					<div class="card-header"><span class="card-title">Integrity</span></div>
					<div class="card-body">
						<div class="field field--stack">
							<span class="field-label">SHA-256 hash</span>
							<span class="field-value mono hash">{data.program.file_hash}</span>
						</div>

						<div class="field field--stack">
							<span class="field-label">WebSocket key</span>
							{#if editing}
								<input
									name="ws_key"
									class="field-input field-input--full"
									bind:value={draft.ws_key}
									placeholder="64-char hex key"
									maxlength="64"
								/>
							{:else}
								<span class="field-value mono hash">{data.program.ws_key}</span>
							{/if}
						</div>

						<div class="field field--stack">
							<span class="field-label">Storage path</span>
							<span class="field-value mono muted">{data.program.storage_path}</span>
						</div>
					</div>
				</div>
			</div>
		</div>
	</form>
</section>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.page {
		display: flex;
		flex-direction: column;
		gap: 24px;
		background: $bg-main;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		border: none;
		border-radius: $radius;
		font-size: 13px;
		font-weight: 600;
		padding: 8px 16px;
		cursor: pointer;
		transition:
			opacity $transition,
			background $transition,
			border-color $transition;
		&--primary {
			background: $accent;
			color: white;
			&:hover {
				opacity: 0.88;
			}
		}
		&--ghost {
			background: $bg-card;
			border: 1px solid $border;
			color: $text-muted;
			&:hover {
				color: $text-primary;
				border-color: $text-muted;
			}
		}
	}

	.save-banner {
		display: flex;
		align-items: center;
		gap: 8px;
		background: rgba(#34d87a, 0.1);
		border: 1px solid rgba(#34d87a, 0.3);
		border-radius: $radius;
		padding: 10px 14px;
		font-size: 13px;
		color: #34d87a;
	}

	.error-banner {
		background: rgba(#f87171, 0.1);
		border: 1px solid rgba(#f87171, 0.3);
		border-radius: $radius;
		padding: 10px 14px;
		font-size: 13px;
		color: #f87171;
	}

	.grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 20px;
		align-items: start;
	}
	.col {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.card {
		background: $bg-sidebar;
		border: 1px solid $border;
		border-radius: $radius;
		overflow: hidden;
	}

	.card-header {
		padding: 12px 18px;
		border-bottom: 1px solid $border;
		background: rgba(0, 0, 0, 0.12);
	}

	.card-title {
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: $text-muted;
	}

	.card-body {
		padding: 16px 18px;
		display: flex;
		flex-direction: column;
		gap: 14px;
	}

	.field {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		min-height: 28px;
		&--stack {
			flex-direction: column;
			align-items: flex-start;
			gap: 6px;
		}
	}

	.field-label {
		font-size: 12px;
		color: $text-muted;
		flex-shrink: 0;
	}

	.field-value {
		font-size: 13px;
		color: $text-primary;
		text-align: right;
		&.mono {
			font-family: 'JetBrains Mono', monospace;
			font-size: 12px;
		}
		&.muted {
			color: $text-muted;
		}
		&.accent {
			color: $accent;
			font-weight: 600;
		}
		&.forbidden {
			color: #f87171;
		}
	}

	.field-input {
		background: $bg-card;
		border: 1px solid $border;
		border-radius: 6px;
		padding: 6px 10px;
		font-size: 13px;
		color: $text-primary;
		outline: none;
		text-align: right;
		width: 200px;
		box-sizing: border-box;
		transition: border-color $transition;
		&:focus {
			border-color: $accent;
		}
		&--full {
			width: 100%;
			text-align: left;
			font-family: 'JetBrains Mono', monospace;
			font-size: 11px;
		}
	}

	.field-right {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 4px;
	}
	.field-hint {
		font-size: 11px;
		color: $text-muted;
	}

	.field-textarea {
		width: 100%;
		background: $bg-card;
		border: 1px solid $border;
		border-radius: 6px;
		padding: 9px 12px;
		font-size: 13px;
		color: $text-primary;
		outline: none;
		resize: vertical;
		font-family: inherit;
		box-sizing: border-box;
		transition: border-color $transition;
		&::placeholder {
			color: $text-muted;
		}
		&:focus {
			border-color: $accent;
		}
	}

	.select-wrap-sm {
		position: relative;
	}

	.field-select {
		appearance: none;
		-webkit-appearance: none;
		background: $bg-card;
		border: 1px solid $border;
		border-radius: 6px;
		padding: 6px 26px 6px 10px;
		font-size: 13px;
		color: $text-primary;
		outline: none;
		cursor: pointer;
		transition: border-color $transition;
		&:focus {
			border-color: $accent;
		}
		option {
			background: $bg-sidebar;
		}
	}

	.chevron {
		position: absolute;
		right: 8px;
		top: 50%;
		transform: translateY(-50%);
		color: $text-muted;
		pointer-events: none;
	}

	.type-preview,
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

	// Read-only capability tags
	.cap-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
	}

	.cap-tag {
		background: $bg-card;
		border: 1px solid $border;
		border-radius: 20px;
		padding: 3px 10px;
		font-size: 11px;
		color: $text-muted;
	}

	.hash {
		word-break: break-all;
		color: $text-muted;
		font-size: 11px;
		line-height: 1.5;
	}
	.description-text {
		margin: 0;
		font-size: 13px;
		color: $text-muted;
		line-height: 1.6;
	}
	.muted-text {
		margin: 0;
		font-size: 13px;
		color: $text-muted;
	}
</style>
