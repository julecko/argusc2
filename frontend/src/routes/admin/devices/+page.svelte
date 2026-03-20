<script lang="ts">
	import PageHeader from '$lib/components/layout/PageHeader.svelte';
</script>

<section class="page">
	<PageHeader title="Devices" description="Manage all devices communicating with c2 server" />
</section>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	section.page {
		display: flex;
		flex-direction: column;
		gap: 16px;
		background: $bg-main;
	}
</style>
