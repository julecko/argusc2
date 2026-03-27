<script lang="ts">
	export let tabs: { id: string; label: string }[];
	export let activeTab: string;
</script>

<div class="tabs">
	{#each tabs as tab}
		<button class="tab" class:active={activeTab === tab.id} on:click={() => (activeTab = tab.id)}>
			{#if activeTab === tab.id}
				<span class="dot"></span>
			{/if}
			{tab.label}
		</button>
	{/each}
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.tabs {
		display: flex;
		align-items: center;
		gap: 2px;
		background: $bg-card;
		border: 1px solid $border;
		border-radius: $radius;
		padding: 4px;
	}

	.tab {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 7px 14px;
		border-radius: 7px;
		border: none;
		background: transparent;
		color: $text-muted;
		font-size: 13px;
		font-weight: 500;
		cursor: pointer;
		transition: all $transition;
		white-space: nowrap;

		&:hover:not(.active) {
			color: $text-primary;
			background: rgba(255, 255, 255, 0.04);
		}

		&.active {
			background: rgba(231, 0, 11, 0.1);
			color: $text-primary;
			border: 1px solid rgba(231, 0, 11, 0.2);
		}

		.dot {
			width: 5px;
			height: 5px;
			border-radius: 50%;
			background: $accent;
			flex-shrink: 0;
		}
	}
</style>
