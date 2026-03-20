<script lang="ts">
	export let title: string;
	export let description: string;
</script>

<div class="page-header">
	<h1>{title}</h1>
	<p>{description}</p>
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.page-header {
		display: flex;
		flex-direction: column;
		gap: 8px;

		h1 {
			color: $text-primary;
			font-size: 30px;
			font-weight: 600;
			margin: 0;
		}

		p {
			color: $text-muted;
			font-size: $text-muted-size;
			margin: 0;
		}
	}
</style>
