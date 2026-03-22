<script lang="ts">
	import type { Capability } from "$lib/types";

	export let options: Capability[] = [];
	export let selected: Set<string> = new Set();

	function toggle(id: string) {
		if (selected.has(id)) {
			selected.delete(id);
		} else {
			selected.add(id);
		}
		selected = new Set(selected);
	}
</script>

<div class="capability-grid">
	{#each options as cap}
		<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
		<div
			class="capability-item"
			class:capability-item--checked={selected.has(cap.id)}
			on:click={() => toggle(cap.id)}
		>
			<div class="checkbox" class:checkbox--checked={selected.has(cap.id)}>
				{#if selected.has(cap.id)}
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
			<div class="capability-text">
				<span class="capability-label">{cap.label}</span>
				<span class="capability-desc">{cap.desc}</span>
			</div>
		</div>
	{/each}
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.capability-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 4px;
		background: $bg-card;
		border: 1px solid $border;
		border-radius: $radius;
		padding: 8px;
	}

	.capability-item {
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

	.capability-text {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.capability-label {
		font-size: 13px;
		font-weight: 500;
		color: $text-primary;
		line-height: 1.2;
	}

	.capability-desc {
		font-size: 11px;
		color: $accent;
		line-height: 1.3;
	}
</style>
