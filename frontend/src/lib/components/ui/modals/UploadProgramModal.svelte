<script lang="ts">
	import Modal from '$components/ui/modals/Modal.svelte';
	import FormField from '$components/ui/form/FormField.svelte';
	import FormRow from '$components/ui/form/FormRow.svelte';
	import SelectInput from '$components/ui/form/SelectInput.svelte';
	import FileDropzone from '$components/ui/form/FileDropzone.svelte';
	import UploadIcon from '$components/ui/icons/UploadIcon.svelte';
	import type { ProgramType, Capability, ProgramDetail } from '$lib/types';

	export let open: boolean = false;
	export let onSuccess: ((program: ProgramDetail) => void) | undefined = undefined;

	let programTypes: ProgramType[] = [];
	let capabilities: Capability[] = [];
	let loadingMeta = false;
	let metaError = '';

	async function loadMeta() {
		loadingMeta = true;
		metaError = '';
		try {
			const token = getToken();
			const headers = { Authorization: `Bearer ${token}` };

			const [typesRes, capsRes] = await Promise.all([
				fetch('/api/program-types/all', { headers }),
				fetch('/api/capabilities/all', { headers })
			]);

			if (typesRes.ok) programTypes = await typesRes.json();
			if (capsRes.ok) capabilities = await capsRes.json();
		} catch {
			metaError = 'Failed to load types.';
		}
		loadingMeta = false;
	}

	// Load when modal opens
	$: if (open && programTypes.length === 0) loadMeta();

	// ── Form state ────────────────────────────────────────────
	let file: File | null = null;
	let fileName: string = '';
	let programName: string = '';
	let version: string = '';
	let typeId: string = '';
	let os: string = 'windows';
	let allowedDownloads: number = -1;
	let description: string = '';
	let wsKey: string = '';
	let selectedCaps: Set<number> = new Set();
	let error: string = '';
	let loading: boolean = false;

	const osList = ['windows', 'linux', 'android', 'mac'];

	// ── Helpers ───────────────────────────────────────────────
	function getToken(): string {
		return (
			document.cookie
				.split('; ')
				.find((c) => c.startsWith('token='))
				?.split('=')[1] ?? ''
		);
	}

	function toggleCap(id: number) {
		if (selectedCaps.has(id)) selectedCaps.delete(id);
		else selectedCaps.add(id);
		selectedCaps = new Set(selectedCaps);
	}

	function handleFileChange(e: CustomEvent<File>) {
		programName = e.detail.name;
	}

	async function generateWsKey() {
		const buf = crypto.getRandomValues(new Uint8Array(32));
		wsKey = Array.from(buf)
			.map((b) => b.toString(16).padStart(2, '0'))
			.join('');
	}

	// ── Validation ────────────────────────────────────────────
	function validate(): string {
		if (!file) return 'Please select a file.';
		if (!programName.trim()) return 'Program name is required.';
		if (!version.trim()) return 'Version is required.';
		if (!typeId) return 'Please select a program type.';
		if (!os) return 'Please select an OS.';
		if (!wsKey.trim()) return 'WebSocket key is required.';
		if (wsKey.length !== 64) return 'WebSocket key must be exactly 64 hex characters.';
		return '';
	}

	// ── Submit ────────────────────────────────────────────────
	async function handleSubmit() {
		error = validate();
		if (error) return;

		loading = true;
		try {
			const fd = new FormData();
			fd.append('file', file!);
			fd.append('name', programName.trim());
			fd.append('version', version.trim());
			fd.append('type_id', typeId);
			fd.append('os', os);
			fd.append('allowed_downloads', String(allowedDownloads));
			fd.append('description', description);
			fd.append('ws_key', wsKey.trim());
			for (const id of selectedCaps) {
				fd.append('capabilities[]', String(id));
			}

			const res = await fetch('/api/programs', {
				method: 'POST',
				headers: { Authorization: `Bearer ${getToken()}` },
				body: fd
			});

			const data = await res.json();
			if (!res.ok) {
				error = data.error ?? 'Upload failed.';
				loading = false;
				return;
			}

			onSuccess?.(data as ProgramDetail);
			close();
		} catch {
			error = 'Could not reach the server.';
			loading = false;
		}
	}

	function close() {
		open = false;
		file = null;
		fileName = '';
		programName = '';
		version = '';
		typeId = '';
		os = 'windows';
		allowedDownloads = -1;
		description = '';
		wsKey = '';
		selectedCaps = new Set();
		error = '';
		loading = false;
	}

	$: if (!open) close();

	$: typeOptions = programTypes.map((t) => t.id.toString());
	$: typeLabels = Object.fromEntries(programTypes.map((t) => [t.id.toString(), t.name]));
</script>

<Modal bind:open title="Upload New Program">
	<svelte:fragment slot="default">
		{#if error}
			<div class="form-error">{error}</div>
		{/if}

		{#if metaError}
			<div class="form-error">{metaError}</div>
		{/if}

		<FileDropzone
			bind:file
			bind:fileName
			label="Select File"
			required
			on:change={handleFileChange}
		/>

		<FormRow>
			<FormField
				id="prog-name"
				label="Program Name"
				required
				bind:value={programName}
				placeholder="e.g., payload.exe"
			/>
			<FormField
				id="prog-ver"
				label="Version"
				required
				bind:value={version}
				placeholder="e.g., 1.0"
			/>
		</FormRow>

		<FormRow>
			<div class="form-field">
				<label class="form-label" for="prog-type">
					Program Type <span class="required">*</span>
				</label>
				<div class="select-wrap">
					<select id="prog-type" class="form-select" bind:value={typeId}>
						<option value="" disabled selected>Select type…</option>
						{#each programTypes as t}
							<option value={String(t.id)}>
								{t.name}
							</option>
						{/each}
					</select>
					<svg
						class="chevron"
						width="12"
						height="12"
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
				<!-- Color preview for selected type -->
				{#if typeId}
					{@const selected = programTypes.find((t) => String(t.id) === typeId)}
					{#if selected}
						<span
							class="type-preview"
							style="color:{selected.color}; background:{selected.color}1a; border-color:{selected.color}40;"
						>
							{selected.name}
						</span>
					{/if}
				{/if}
			</div>

			<SelectInput
				id="prog-os"
				label="Operating System"
				required
				bind:value={os}
				options={osList}
			/>
		</FormRow>

		<!-- Allowed downloads — -1 = unlimited, 0 = forbidden -->
		<div class="form-field">
			<label class="form-label" for="prog-dl">
				Allowed Downloads
				<span class="form-hint">(-1 = unlimited · 0 = forbidden · &gt;0 = fixed limit)</span>
			</label>
			<input id="prog-dl" class="form-input" type="number" bind:value={allowedDownloads} />
		</div>

		<!-- WebSocket key -->
		<div class="form-field">
			<label class="form-label" for="prog-wskey">
				WebSocket Key <span class="required">*</span>
				<span class="form-hint">(64-char hex)</span>
			</label>
			<div class="wskey-row">
				<input
					id="prog-wskey"
					class="form-input wskey-input"
					type="text"
					maxlength="64"
					placeholder="64-character hex key…"
					bind:value={wsKey}
					spellcheck="false"
				/>
				<button type="button" class="generate-btn" on:click={generateWsKey}>Generate</button>
			</div>
			{#if wsKey}
				<span class="wskey-len" class:ok={wsKey.length === 64} class:bad={wsKey.length !== 64}
					>{wsKey.length} / 64</span
				>
			{/if}
		</div>

		<!-- Capabilities from DB -->
		{#if capabilities.length > 0}
			<div class="form-field">
				<span class="form-label">Capabilities</span>
				<div class="cap-grid">
					{#each capabilities as cap}
						<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
						<div
							class="cap-item"
							class:cap-item--checked={selectedCaps.has(cap.id)}
							on:click={() => toggleCap(cap.id)}
						>
							<div class="checkbox" class:checkbox--checked={selectedCaps.has(cap.id)}>
								{#if selectedCaps.has(cap.id)}
									<svg
										width="10"
										height="10"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="3"
										stroke-linecap="round"
										stroke-linejoin="round"
									>
										<polyline points="20 6 9 17 4 12" />
									</svg>
								{/if}
							</div>
							<div class="cap-text">
								<span class="cap-label">{cap.label}</span>
								{#if cap.description}
									<span class="cap-desc">{cap.description}</span>
								{/if}
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<div class="form-field">
			<label class="form-label" for="prog-desc">Description</label>
			<textarea
				id="prog-desc"
				class="form-textarea"
				rows="3"
				placeholder="Enter program description…"
				bind:value={description}
			></textarea>
		</div>
	</svelte:fragment>

	<svelte:fragment slot="footer">
		<button
			class="submit-btn"
			class:submit-btn--loading={loading}
			on:click={handleSubmit}
			disabled={loading}
		>
			{#if loading}
				<span class="spinner"></span>
			{:else}
				<UploadIcon size={15} />
				Upload Program
			{/if}
		</button>
	</svelte:fragment>
</Modal>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.form-field {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.form-label {
		font-size: 12px;
		font-weight: 500;
		color: $text-muted;
		.required {
			color: $accent;
		}
	}

	.form-hint {
		font-weight: 400;
		color: $text-muted;
	}

	.form-input,
	.form-textarea {
		background: $bg-card;
		border: 1px solid $border;
		border-radius: $radius;
		padding: 9px 12px;
		font-size: 13px;
		color: $text-primary;
		outline: none;
		width: 100%;
		box-sizing: border-box;
		transition: border-color $transition;
		&::placeholder {
			color: $text-muted;
		}
		&:focus {
			border-color: $accent;
		}
	}

	.form-textarea {
		resize: vertical;
		min-height: 72px;
		font-family: inherit;
	}

	// ── Type select ───────────────────────────────────────────
	.select-wrap {
		position: relative;
	}

	.form-select {
		appearance: none;
		-webkit-appearance: none;
		width: 100%;
		background: $bg-card;
		border: 1px solid $border;
		border-radius: $radius;
		padding: 9px 32px 9px 12px;
		font-size: 13px;
		color: $text-primary;
		outline: none;
		cursor: pointer;
		box-sizing: border-box;
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
		right: 10px;
		top: 50%;
		transform: translateY(-50%);
		color: $text-muted;
		pointer-events: none;
	}

	.type-preview {
		display: inline-flex;
		align-items: center;
		padding: 2px 10px;
		border-radius: 20px;
		font-size: 11px;
		font-weight: 600;
		border: 1px solid transparent;
		align-self: flex-start;
	}

	// ── WS key ────────────────────────────────────────────────
	.wskey-row {
		display: flex;
		gap: 8px;
	}

	.wskey-input {
		flex: 1;
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
	}

	.generate-btn {
		flex-shrink: 0;
		background: $bg-card;
		border: 1px solid $border;
		border-radius: $radius;
		color: $text-primary;
		font-size: 12px;
		font-weight: 600;
		padding: 0 14px;
		cursor: pointer;
		white-space: nowrap;
		transition:
			border-color $transition,
			background $transition;
		&:hover {
			border-color: $accent;
			background: rgba($accent, 0.06);
		}
	}

	.wskey-len {
		font-size: 11px;
		&.ok {
			color: #34d87a;
		}
		&.bad {
			color: #f5a623;
		}
	}

	// ── Capabilities ──────────────────────────────────────────
	.cap-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 4px;
		background: $bg-card;
		border: 1px solid $border;
		border-radius: $radius;
		padding: 8px;
	}

	.cap-item {
		display: flex;
		align-items: flex-start;
		gap: 10px;
		padding: 10px;
		border-radius: 6px;
		cursor: pointer;
		transition: background $transition;
		&:hover {
			background: rgba(255, 255, 255, 0.04);
		}
		&--checked {
			background: rgba($accent, 0.05);
		}
	}

	.checkbox {
		width: 16px;
		height: 16px;
		border-radius: 4px;
		border: 1px solid $border;
		background: $bg-main;
		flex-shrink: 0;
		margin-top: 1px;
		display: flex;
		align-items: center;
		justify-content: center;
		transition:
			background $transition,
			border-color $transition;
		&--checked {
			background: $accent;
			border-color: $accent;
			color: white;
		}
	}

	.cap-text {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}
	.cap-label {
		font-size: 13px;
		font-weight: 500;
		color: $text-primary;
		line-height: 1.2;
	}
	.cap-desc {
		font-size: 11px;
		color: $accent;
		line-height: 1.3;
	}

	.form-error {
		background: rgba(#f87171, 0.1);
		border: 1px solid rgba(#f87171, 0.3);
		border-radius: 6px;
		padding: 10px 14px;
		font-size: 13px;
		color: #f87171;
	}

	.submit-btn {
		width: 100%;
		padding: 11px;
		background: $accent;
		border: none;
		border-radius: $radius;
		color: white;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		transition:
			opacity $transition,
			transform $transition;
		&:hover:not(:disabled) {
			opacity: 0.88;
		}
		&:active:not(:disabled) {
			transform: scale(0.98);
		}
		&:disabled {
			cursor: not-allowed;
			opacity: 0.7;
		}
		&--loading {
			pointer-events: none;
		}
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(white, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
