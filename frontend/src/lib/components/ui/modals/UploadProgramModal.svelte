<script lang="ts">
	import Modal from '$components/ui/modals/Modal.svelte';
	import FormField from '$components/ui/form/FormField.svelte';
	import FormRow from '$components/ui/form/FormRow.svelte';
	import SelectInput from '$components/ui/form/SelectInput.svelte';
	import FileDropzone from '$lib/components/ui/form/FileDropzone.svelte';
	import CapabilityGrid from '$components/ui/form/CapabilityGrid.svelte';
	import UploadIcon from '$components/ui/icons/UploadIcon.svelte';
	import type { Capability } from '$lib/types';

	export let open: boolean = false;

	// ── Form state ────────────────────────────────────────────
	let file: File | null = null;
	let fileName: string = '';
	let programName: string = '';
	let version: string = '';
	let programType: string = '';
	let os: string = 'Windows';
	let allowedDownloads: number = -1;
	let description: string = '';
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

	const programTypes = ['RAT', 'Keylogger', 'Spyware', 'Ransomware', 'Trojan', 'Worm', 'Other'];
	const osList = ['Windows', 'Linux', 'macOS', 'Android', 'iOS'];

	function handleFileChange(e: CustomEvent<File>) {
		programName = e.detail.name;
	}

	function validate(): string {
		if (!file) return 'Please select a file.';
		if (!programName.trim()) return 'Program name is required.';
		if (!version.trim()) return 'Version is required.';
		if (!programType) return 'Please select a program type.';
		return '';
	}

	async function handleSubmit() {
		error = validate();
		if (error) return;

		loading = true;
		// TODO: POST to /api/programs with FormData
		await new Promise((r) => setTimeout(r, 800));
		loading = false;
		close();
	}

	function close() {
		open = false;
		file = null;
		fileName = '';
		programName = '';
		version = '';
		programType = '';
		os = 'Windows';
		allowedDownloads = -1;
		description = '';
		capabilities = new Set();
		error = '';
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
				bind:value={programType}
				options={programTypes}
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
				Allowed Downloads <span class="form-hint">(-1 = unlimited)</span>
			</label>
			<input id="prog-dl" class="form-input" type="number" bind:value={allowedDownloads} />
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
	}

	.form-hint {
		color: $text-muted;
		font-weight: 400;
	}

	.form-input {
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
		background: $bg-card;
		border: 1px solid $border;
		border-radius: $radius;
		padding: 9px 12px;
		font-size: 13px;
		color: $text-primary;
		outline: none;
		width: 100%;
		box-sizing: border-box;
		resize: vertical;
		min-height: 72px;
		font-family: inherit;
		transition: border-color $transition;

		&::placeholder {
			color: $text-muted;
		}
		&:focus {
			border-color: $accent;
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
