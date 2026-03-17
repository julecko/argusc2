<script lang="ts">
	export let size: number = 18;
	export let padding: number = 8;
</script>

<div class="icon">
	<svg
		viewBox="0 0 24 24"
		fill="none"
		stroke="currentColor"
		stroke-width="2"
		stroke-linecap="round"
		stroke-linejoin="round"
		style={`width:${size}px; height:${size}px; padding: ${padding}px;`}
	>
		<polyline points="4 17 10 11 4 5"></polyline>
		<line x1="12" x2="20" y1="19" y2="19"></line>
	</svg>
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.icon {
		display: flex;
		align-items: center;
		justify-content: center;
		background: $accent;
		border-radius: 4px;
		color: white;
	}
</style>
