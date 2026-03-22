<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let file: File | null = null;
	export let fileName: string = '';
	export let label: string = 'Select File';
	export let required: boolean = false;

	const dispatch = createEventDispatcher<{ change: File }>();

	let dragging = false;

	function handleFileChange(e: Event) {
		const input = e.target as HTMLInputElement;
		if (input.files?.[0]) {
			file = input.files[0];
			fileName = file.name;
			dispatch('change', file);
		}
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		dragging = false;
		const dropped = e.dataTransfer?.files[0];
		if (dropped) {
			file = dropped;
			fileName = dropped.name;
			dispatch('change', dropped);
		}
	}
</script>

<div class="form-field">
	{#if label}
		<!-- svelte-ignore a11y_label_has_associated_control -->
		<label class="form-label">
			{label}
			{#if required}<span class="required">*</span>{/if}
		</label>
	{/if}

	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div
		class="dropzone"
		class:dropzone--active={dragging}
		class:dropzone--filled={!!file}
		on:dragover|preventDefault={() => (dragging = true)}
		on:dragleave={() => (dragging = false)}
		on:drop={handleDrop}
	>
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
			<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
			<polyline points="17 8 12 3 7 8" />
			<line x1="12" y1="3" x2="12" y2="15" />
		</svg>
		<span>{fileName || 'Browse… No file selected.'}</span>
		<input type="file" class="file-input" on:change={handleFileChange} aria-label="Select file" />
	</div>
</div>

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

	.dropzone {
		position: relative;
		display: flex;
		align-items: center;
		gap: 10px;
		background: $bg-card;
		border: 1px dashed $border;
		border-radius: $radius;
		padding: 11px 14px;
		font-size: 13px;
		color: $text-muted;
		cursor: pointer;
		transition:
			border-color $transition,
			background $transition,
			color $transition;

		&:hover,
		&--active {
			border-color: $accent;
			background: rgba($accent, 0.04);
			color: $text-primary;
		}

		&--filled {
			border-style: solid;
			border-color: $accent;
			color: $text-primary;
		}
	}

	.file-input {
		position: absolute;
		inset: 0;
		opacity: 0;
		cursor: pointer;
		width: 100%;
		height: 100%;
	}
</style>
