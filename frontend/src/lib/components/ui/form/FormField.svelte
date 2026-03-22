<script lang="ts">
	export let label: string = '';
	export let id: string = '';
	export let value: string = '';
	export let placeholder: string = '';
	export let type: string = 'text';
	export let required: boolean = false;
	export let disabled: boolean = false;
</script>

<div class="form-field">
	{#if label}
		<label class="form-label" for={id}>
			{label}
			{#if required}<span class="required">*</span>{/if}
		</label>
	{/if}
	<input {id} {type} {placeholder} {disabled} bind:value class="form-input" on:keydown />
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
		&:disabled {
			opacity: 0.5;
			cursor: not-allowed;
		}
	}
</style>
