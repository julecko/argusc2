<script lang="ts">
	export let columns: number = 2;
</script>

<div class="form-row" style="grid-template-columns: repeat({columns}, 1fr)">
	<slot />
</div>

<style lang="scss">
	.form-row {
		display: grid;
		gap: 12px;
	}
</style>
