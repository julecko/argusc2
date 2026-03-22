<script lang="ts">
	export let id: string = '';
	export let label: string = '';
	export let value: string = '';
	export let options: string[] = [];
	export let placeholder: string = '';
	export let required: boolean = false;
</script>

<div class="form-field">
	{#if label}
		<label class="form-label" for={id}>
			{label}
			{#if required}<span class="required">*</span>{/if}
		</label>
	{/if}
	<div class="select-wrap">
		<select {id} class="form-select" bind:value>
			{#if placeholder}
				<option value="" disabled selected>{placeholder}</option>
			{/if}
			{#each options as opt}
				<option value={opt}>{opt}</option>
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
</style>
