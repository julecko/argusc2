<script lang="ts">
	import PageHeader from '$lib/components/layout/PageHeader.svelte';
</script>

<section class="page">
	<PageHeader title="Accounts" description="Manage user accounts and permissions" />
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
